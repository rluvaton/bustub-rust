use crate::cli::Args;
use crate::metrics::bpm_metrics::BpmMetrics;
use crate::page_process::{check_page_consistent, check_page_consistent_no_seed, modify_page};
use db_core::buffer::{AccessType, BufferPool, BufferPoolManager};
use clap::Parser;
use common::config::PageId;
use metrics::bpm_total_metrics::BpmTotalMetrics;
use parking_lot::{Mutex, RwLock};
use rand::distributions::Distribution;
use std::collections::HashMap;
use std::process::abort;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
#[allow(unused)]
use db_core::storage::{DefaultDiskManager, DiskManagerUnlimitedMemory};
use tempdir::TempDir;

#[cfg(feature = "tracing")]
use tracy_client::*;

// Tracking Memory usage
#[cfg(feature = "tracing-memory-allocation")]
#[global_allocator]
static GLOBAL: ProfiledAllocator<std::alloc::System> =
    ProfiledAllocator::new(std::alloc::System, 100);

mod cli;
mod page_process;
mod metrics;

// Single lock for bpm
// Debug:
// <<< BEGIN
// scan: 20211.485900939937
// get: 20915.072328511433
// >>> END
//
// Release:
// <<< BEGIN
// scan: 61088.29705676477
// get: 68200.29332355589
// >>> END
//
// Release with latency:
// <<< BEGIN
// scan: 230.8101990546568
// get: 255.20937354370548
// >>> END

// Single lock for BPM but no Arc and mutex in the benchmark itself
// Debug
// <<< BEGIN
// scan: 21303.056564781175
// get: 19764.5411819606
// >>> END
//
// Release:
// <<< BEGIN
// scan: 58489.01703276557
// get: 43897.136762107926
// >>> END
//
// Release with latency:
// <<< BEGIN
// scan: 241.85860471317622
// get: 253.25822472584247
// >>> END

// Fine granularity lock
// Debug:
// <<< BEGIN
// scan: 19685.5104829839
// get: 17487.217092763574
// >>> END
//
// Release:
// <<< BEGIN
// scan: 34082.597246758436
// get: 32525.449151694942
// >>> END
//
// Release with latency:
// <<< BEGIN
// scan: 189.69079035052647
// get: 187.25842996134878
// >>> END
//
// Release without latency and without cloning data:
// <<< BEGIN
// scan: 39509.91633612213
// get: 36550.64831172294
// >>> END


// Fine granularity lock with updated lru-k
// Debug:
// <<< BEGIN
// scan: 31941.968601046632
// get: 33355.45481817272
// >>> END
//
// Release:
// <<< BEGIN
// scan: 134585.84713842874
// get: 149108.7963734542
// >>> END

// Before try to clean up 9d5cd7377f0efa2152f51bf8417db1e61f0eef59
// Release:
// <<< BEGIN
// scan: 147529.04903169896
// get: 163663.34455518148
// >>> END

// After avoid using timestamp in LRU-K
// Release
// <<< BEGIN
// scan: 160378.7207093097
// get: 177758.30805639812
// >>> END

// After avoiding updating tree twice
// Release
// <<< BEGIN
// scan: 162523.1825605813
// get: 180701.67661077963
// >>> END

// After replacing map store in LRU-K with simple array
// Release
// <<< BEGIN
// scan: 166284.39052031597
// get: 184767.807739742
// >>> END

// After Creating mut binary heap with identity hash function
// Release
// <<< BEGIN
// scan: 167936.7687743742
// get: 187041.3319556015
// >>> END

// After cloning mut_binary_heap and modifying
// Release:
// <<< BEGIN
// scan: 179341.45528482384
// get: 199197.69341021968
// >>> END

// After reusing LRU-K nodes
// Release:
// <<< BEGIN
// scan: 199292.02359921337
// get: 221965.43448551712
// >>> END

// After avoiding saving option in lru and always reuse + avoid is_evictable memory
// Release:
// <<< BEGIN
// scan: 211548.6634224385
// get: 236335.41097260182
// >>> END



fn setup() -> TempDir {
    TempDir::new("bpm_bench").expect("Should create tmp directory")
}

fn main() {
    #[cfg(feature = "tracing")]
    let client = Client::start();

    let args = Args::parse();

    println!("args: {:?}", args);

    let duration_ms = args.duration;
    let enable_latency = args.latency;
    let scan_thread_n = args.scan_thread_n;
    let get_thread_n = args.get_thread_n;
    let bustub_page_cnt = args.db_size;
    let bustub_bpm_size = args.bpm_size;
    let lru_k_size = args.lru_k_size;

    println!("[info] total_page={}, duration_ms={}, latency={}, lru_k_size={}, bpm_size={}, scan_thread_cnt={}, get_thread_cnt={}",
             bustub_page_cnt, duration_ms, enable_latency, lru_k_size, bustub_bpm_size, scan_thread_n, get_thread_n
    );


    let tmpdir = setup();

    #[allow(unused)]
    let db_name = tmpdir.path().join("test.db");

    // let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
    let disk_manager = DiskManagerUnlimitedMemory::new();


    let disk_manager = Arc::new(Mutex::new(disk_manager));

    let bpm: Arc<BufferPoolManager> = BufferPoolManager::new(
        bustub_bpm_size,
        Arc::clone(&disk_manager),
        Some(lru_k_size),
        None, /* log manager */
    );
    let page_ids: Arc<RwLock<Vec<PageId>>> = Arc::new(RwLock::new(vec![]));

    init_pages(bustub_page_cnt, &bpm, &page_ids);
    // validate_initialized_pages(bustub_page_cnt, &bpm, &page_ids);


    // enable disk latency after creating all pages
    // disk_manager.lock().enable_latency_simulator(enable_latency);

    println!("[info] benchmark start");

    let total_metrics = Arc::new(Mutex::new(BpmTotalMetrics::new()));

    total_metrics.lock().begin();

    let mut join_handles: Vec<JoinHandle<()>> = vec![];
    type ModifyRecord = HashMap<PageId, u64>;

    for thread_id in 0..scan_thread_n {
        let bpm = Arc::clone(&bpm);
        let total_metrics = Arc::clone(&total_metrics);
        let page_ids = Arc::clone(&page_ids);

        let t = thread::spawn(move || {
            let mut records: ModifyRecord = HashMap::new();

            let mut metrics: BpmMetrics = BpmMetrics::new(format!("scan {:>2}", thread_id), duration_ms);
            metrics.begin();
            let page_idx_start = (bustub_page_cnt * thread_id / scan_thread_n) as PageId;
            let page_idx_end = (bustub_page_cnt * (thread_id + 1) / scan_thread_n) as PageId;
            let mut page_idx = page_idx_start;

            while !metrics.should_finish() {
                #[cfg(feature = "tracing")]
                let _scan_iteration = span!("scan thread");


                let page = {
                    #[cfg(feature = "tracing")]
                    let _fetch_page = span!("scan thread - fetch page");
                    bpm.fetch_page_write(page_ids.read()[page_idx as usize], AccessType::Scan)
                };

                if page.is_err() {
                    continue;
                }

                let mut page = page.unwrap();


                {
                    #[cfg(feature = "tracing")]
                    let _update_page = span!("scan thread - Updating page");

                    if !records.contains_key(&page_idx) {
                        records.insert(page_idx, 0);
                    }

                    let seed = records.get_mut(&page_idx).expect("Must exists");

                    check_page_consistent(page.get_data(), page_idx as usize, *seed);
                    *seed = *seed + 1;
                    modify_page(page.get_data_mut(), page_idx as usize, *seed);
                }

                {
                    #[cfg(feature = "tracing")]
                    let _unpin_page_span = span!("scan thread - Unpin page");
                    drop(page);
                }

                page_idx += 1;
                if page_idx >= page_idx_end {
                    page_idx = page_idx_start;
                }
                metrics.tick();
                metrics.report();
            }

            total_metrics.lock().report_scan(metrics.count);
        });
        join_handles.push(t);
    }

    for thread_id in 0..get_thread_n {
        let bpm = Arc::clone(&bpm);

        let total_metrics = Arc::clone(&total_metrics);
        let page_ids = Arc::clone(&page_ids);

        let t = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let dist = zipf::ZipfDistribution::new(bustub_page_cnt - 1, 0.8).unwrap();

            let mut metrics: BpmMetrics = BpmMetrics::new(format!("get  {:>2}", thread_id), duration_ms);
            metrics.begin();

            while !metrics.should_finish() {
                #[cfg(feature = "tracing")]
                let _span = span!("get thread");

                let page_idx = dist.sample(&mut rng);



                let page = {
                    #[cfg(feature = "tracing")]
                    let _fetch_page_span = span!("get thread - fetch page");
                    bpm.fetch_page_read(page_ids.read()[page_idx], AccessType::Lookup)
                };

                if page.is_err() {
                    eprintln!("cannot fetch page");
                    abort();
                }

                let page = page.unwrap();

                {
                    #[cfg(feature = "tracing")]
                    let _verify_page = span!("get thread - verify page");

                    check_page_consistent_no_seed(page.get_data(), page_idx);
                }
                {
                    #[cfg(feature = "tracing")]
                    let _unpin_page_span = span!("get thread - Unpin page");
                    drop(page);
                }

                metrics.tick();
                metrics.report();
            }

            total_metrics.lock().report_get(metrics.count);
        });
        join_handles.push(t);
    }

    for t in join_handles {
        t.join().unwrap();
    }

    total_metrics.lock().report();

    println!("\n\n");
    println!("###################");
    println!("       Stats       ");
    println!("###################");
    println!();
    #[cfg(feature = "statistics")]
    println!("{}", bpm.clone().get_stats());
}

fn init_pages(bustub_page_cnt: usize, bpm: &Arc<BufferPoolManager>, page_ids: &Arc<RwLock<Vec<PageId>>>) {
    #[cfg(feature = "tracing")]
    let _span = span!("init pages");

    let current_page_index = Arc::new(Mutex::new(0usize));

    let mut join_handles: Vec<JoinHandle<()>> = vec![];

    for _ in 0..1000 {
        let bpm = Arc::clone(&bpm);
        let page_ids = Arc::clone(&page_ids);

        let current_page_index = Arc::clone(&current_page_index);

        let t = thread::spawn(move || {

            let mut guard = current_page_index.lock();
            let mut i = guard.clone();

            while i < bustub_page_cnt {
                *guard += 1;

                let mut page = bpm.new_page(AccessType::Unknown).expect("Must be able to create a page");
                let page_id = page.get_page_id();
                page_ids.write().push(page_id);

                modify_page(page.get_data_mut(), i, 0);

                drop(guard);

                drop(page);

                guard = current_page_index.lock();

                i = guard.clone();
            }
        });

        join_handles.push(t);
    }

    for handle in join_handles {
        handle.join().unwrap()
    }
}


#[allow(unused)]
fn validate_initialized_pages(bustub_page_cnt: usize, bpm: &Arc<BufferPoolManager>, page_ids: &Arc<RwLock<Vec<PageId>>>) {
    println!("Validate pages");

    for page_idx in 0..bustub_page_cnt {
        if page_idx % (bustub_page_cnt / 1000) == 0 {
            println!("{}/{}", page_idx, bustub_page_cnt);
        }

        let page = bpm.fetch_page_read(page_ids.read()[page_idx], AccessType::Lookup);

        if page.is_err() {
            eprintln!("cannot fetch page");
            abort();
        }

        let page = page.unwrap();

        check_page_consistent_no_seed(page.get_data(), page_idx);
    }

    println!("Validation finish");
}
