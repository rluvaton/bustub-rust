use std::collections::HashMap;
use std::process::abort;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use clap::Parser;
use parking_lot::{Mutex, RwLock};
use rand::distributions::Distribution;
use buffer::{AccessType, BufferPoolManager};
use common::config::PageId;
use storage::DiskManagerUnlimitedMemory;
use metrics::bpm_total_metrics::BpmTotalMetrics;
use crate::cli::Args;
use crate::metrics::bpm_metrics::BpmMetrics;
use crate::page_process::{check_page_consistent, check_page_consistent_no_seed, modify_page};

mod cli;
mod page_process;
mod metrics;

// Single lock for bpm
// <<< BEGIN
// scan: 20211.485900939937
// get: 20915.072328511433
// >>> END

// Single lock for BPM but no Arc and mutex in the benchmark itself
// <<< BEGIN
// scan: 21303.056564781175
// get: 19764.5411819606
// >>> END

fn main() {
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

    let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
    let bpm: BufferPoolManager = BufferPoolManager::new(
        bustub_bpm_size,
        Arc::clone(&disk_manager),
        Some(lru_k_size),
        None, /* log manager */
    );
    let page_ids: Arc<RwLock<Vec<PageId>>> = Arc::new(RwLock::new(vec![]));

    for i in 0..bustub_page_cnt {
        let page = bpm.new_page().expect("Must be able to create a page");
        let page_id = page.with_read(|u| u.get_page_id());

        page.with_write(|u| unsafe {
            modify_page(u.get_data_mut(), i, 0);
        });

        bpm.unpin_page(page_id, true, AccessType::default());

        page_ids.write().push(page_id);
    }


    // enable disk latency after creating all pages
    disk_manager.lock().enable_latency_simulator(enable_latency);

    println!("[info] benchmark start");

    let total_metrics = Arc::new(Mutex::new(BpmTotalMetrics::new()));

    total_metrics.lock().begin();

    let mut join_handles: Vec<JoinHandle<()>> = vec![];
    type ModifyRecord = HashMap<PageId, u64>;

    for thread_id in 0..scan_thread_n {
        let bpm = bpm.clone();
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

                let page = bpm.fetch_page(page_ids.read()[page_idx as usize], AccessType::Scan);
                if page.is_none() {
                    continue;
                }

                let page = page.unwrap();

                page.with_write(|u| unsafe {
                    if !records.contains_key(&page_idx) {
                        records.insert(page_idx, 0);
                    }

                    let mut seed = records.get_mut(&page_idx).expect("Must exists");

                    check_page_consistent(u.get_data(), page_idx as usize, *seed);
                    *seed = *seed + 1;
                    modify_page(u.get_data_mut(), page_idx as usize, *seed);
                });

                bpm.unpin_page(page.with_read(|u| u.get_page_id()), true, AccessType::Scan);
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
        let bpm = bpm.clone();

        let total_metrics = Arc::clone(&total_metrics);
        let page_ids = Arc::clone(&page_ids);

        let t = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let dist = zipf::ZipfDistribution::new(bustub_page_cnt - 1, 0.8).unwrap();

            let mut metrics: BpmMetrics = BpmMetrics::new(format!("get  {:>2}", thread_id), duration_ms);
            metrics.begin();

            while !metrics.should_finish() {

                let page_idx = dist.sample(&mut rng);
                let page = bpm.fetch_page(page_ids.read()[page_idx], AccessType::Lookup);

                if page.is_none() {
                    eprintln!("cannot fetch page");
                    abort();
                }

                let page = page.unwrap();

                let mut page_id: PageId = 0;

                page.with_read(|u| unsafe {
                    page_id = u.get_page_id();
                    check_page_consistent_no_seed(u.get_data(), page_idx);
                });

                bpm.unpin_page(page_id, false, AccessType::Lookup);

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
}

