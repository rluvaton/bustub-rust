use std::collections::HashMap;
use std::fmt::format;
use std::process::abort;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::thread::JoinHandle;
use clap::Parser;
use parking_lot::{Mutex, RwLock};
use rand::distributions::Distribution;
use tempdir::TempDir;
use buffer::{AccessType, BufferPoolManager};
use common::config::PageId;
use storage::{DefaultDiskManager, DiskManagerUnlimitedMemory};
use metrics::bpm_total_metrics::BpmTotalMetrics;
use crate::cli::Args;
use crate::metrics::bpm_metrics::BpmMetrics;
use crate::page_process::{check_page_consistent, check_page_consistent_no_seed, modify_page};
use tracy_client::*;

// Tracking Memory usage
// #[global_allocator]
// static GLOBAL: ProfiledAllocator<std::alloc::System> =
//     ProfiledAllocator::new(std::alloc::System, 100);

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


fn setup() -> TempDir {
    TempDir::new("bpm_bench").expect("Should create tmp directory")
}

fn main() {
    let client = Client::start();

    let args = Args::parse();

    client.message("starting", 10);

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
    let db_name = tmpdir.path().join("test.db");

    // let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
    let disk_manager = DiskManagerUnlimitedMemory::new();


    let disk_manager = Arc::new(Mutex::new(disk_manager));

    let bpm: Arc<BufferPoolManager> = Arc::new(BufferPoolManager::new(
        bustub_bpm_size,
        Arc::clone(&disk_manager),
        Some(lru_k_size),
        None, /* log manager */
    ));
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
                let scan_iteration = span!("scan thread");

                let fetch_page = span!("scan thread - fetch page");
                let page = bpm.fetch_page(page_ids.read()[page_idx as usize], AccessType::Scan);
                drop(fetch_page);

                if page.is_none() {
                    continue;
                }

                let page = page.unwrap();


                {
                    let _update_page = span!("scan thread - Updating page");

                    let get_write_lock_page = span!("scan thread - Acquiring write lock");

                    page.with_write(|u| unsafe {
                        drop(get_write_lock_page);

                        if !records.contains_key(&page_idx) {
                            records.insert(page_idx, 0);
                        }

                        let mut seed = records.get_mut(&page_idx).expect("Must exists");

                        check_page_consistent(u.get_data(), page_idx as usize, *seed);
                        *seed = *seed + 1;
                        modify_page(u.get_data_mut(), page_idx as usize, *seed);
                    });
                }

                {
                    let _unpin_page_span = span!("scan thread - Unpin page");
                    bpm.unpin_page(page.with_read(|u| u.get_page_id()), true, AccessType::Scan);
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
                let span = span!("get thread");

                let page_idx = dist.sample(&mut rng);


                let fetch_page_span = span!("get thread - fetch page");
                let page = bpm.fetch_page(page_ids.read()[page_idx], AccessType::Lookup);
                drop(fetch_page_span);

                if page.is_none() {
                    eprintln!("cannot fetch page");
                    abort();
                }

                let page = page.unwrap();

                let mut page_id: PageId = 0;

                {
                    let _verify_page = span!("get thread - verify page");

                    let get_read_lock_page = span!("get thread - Acquiring read lock");

                    page.with_read(|u| unsafe {
                        drop(get_read_lock_page);

                        page_id = u.get_page_id();
                        check_page_consistent_no_seed(u.get_data(), page_idx);
                    });
                }
                {
                    let _unpin_page_span = span!("get thread - Unpin page");
                    bpm.unpin_page(page_id, false, AccessType::Lookup);
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
    println!("{}", bpm.clone().get_stats());
}

fn init_pages(bustub_page_cnt: usize, bpm: &Arc<BufferPoolManager>, page_ids: &Arc<RwLock<Vec<PageId>>>) {
    let _span = span!("init pages");

    let current_page_index = Arc::new(Mutex::new(0usize));

    let mut join_handles: Vec<JoinHandle<()>> = vec![];
    type ModifyRecord = HashMap<PageId, u64>;

    for _ in 0..1000 {
        let bpm = Arc::clone(&bpm);
        let page_ids = Arc::clone(&page_ids);

        let current_page_index = Arc::clone(&current_page_index);

        let t = thread::spawn(move || {

            let mut guard = current_page_index.lock();
            let mut i = guard.clone();

            while i < bustub_page_cnt {
                *guard += 1;

                let page = bpm.new_page().expect("Must be able to create a page");
                let page_id = page.with_read(|u| u.get_page_id());
                page_ids.write().push(page_id);

                page.with_write(|u| unsafe {
                    modify_page(u.get_data_mut(), i, 0);
                });

                drop(guard);

                bpm.unpin_page(page_id, true, AccessType::default());

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


fn validate_initialized_pages(bustub_page_cnt: usize, bpm: &Arc<BufferPoolManager>, page_ids: &Arc<RwLock<Vec<PageId>>>) {
    println!("Validate pages");

    for page_idx in 0..bustub_page_cnt {
        if page_idx % (bustub_page_cnt / 1000) == 0 {
            println!("{}/{}", page_idx, bustub_page_cnt);
        }

        let page = bpm.fetch_page(page_ids.read()[page_idx], AccessType::Lookup);

        if page.is_none() {
            eprintln!("cannot fetch page");
            abort();
        }

        let page = page.unwrap();

        let mut page_id: PageId = 0;

        page.with_read(|u| {
            page_id = u.get_page_id();
            check_page_consistent_no_seed(u.get_data(), page_idx);
        });

        bpm.unpin_page(page_id, false, AccessType::Lookup);
    }

    println!("Validation finish");
}
