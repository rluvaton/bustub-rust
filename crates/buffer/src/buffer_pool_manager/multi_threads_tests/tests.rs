extern crate derive_builder;

use crate::{AccessType, BufferPoolManager};
use common::config::{PageData, PageId};
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use storage::{DefaultDiskManager, DiskManager, DiskManagerUnlimitedMemory};
use tempdir::TempDir;

use crate::buffer_pool_manager::multi_threads_tests::helpers::{get_tmp_dir, RunTimer};
use crate::buffer_pool_manager::multi_threads_tests::options::{DiskManagerImplementationOptions, Options};
use rand::distributions::Distribution;

// This is the structure of the page
struct BustubBenchPageHeader {
    seed: u64,
    page_id: u64,

    // Until end of page data
    data: &'static [u8],
}

fn modify_page(page: &mut PageData, page_idx: usize, seed: u64) {
    page[0..8].copy_from_slice(&u64::to_ne_bytes(seed));
    page[8..16].copy_from_slice(&u64::to_ne_bytes(page_idx as u64));
    page[16 + (seed as usize % 4000)] = (seed % 256) as u8;
}

/// Check the page and verify the data inside
fn check_page_consistent_no_seed(data: &PageData, page_idx: usize) {
    // Cast the data pointer to a BustubBenchPageHeader pointer
    let data_seed = u64::from_ne_bytes(data[0..8].try_into().unwrap());
    let data_page_id = u64::from_ne_bytes(data[8..16].try_into().unwrap());

    if data_page_id != page_idx as u64 {
        eprintln!("Page header not consistent: page_id={} page_idx={}", data_page_id, page_idx);
        panic!();
    }

    let left = data[(16 + (data_seed % 4000)) as usize];
    let right = (data_seed % 256) as u8;

    // Check if the data content is consistent
    if left != right {
        eprintln!(
            "page content not consistent: data[{}]={} seed % 256={}",
            (16 + (data_seed % 4000)),
            left,
            right
        );
        panic!();
    }
}

fn check_page_consistent(data: &PageData, page_idx: usize, seed: u64) {
    let data_seed = u64::from_ne_bytes(data[0..8].try_into().unwrap());

    // Check if the seed matches the expected seed
    if data_seed != seed {
        eprintln!(
            "{} page seed not consistent: page.seed={} seed={}",
            page_idx, data_seed, seed
        );
        panic!();
    }

    check_page_consistent_no_seed(data, page_idx);
}

fn run_multi_threads_tests(options: Options) {
    let duration_ms = options.duration_ms;
    let scan_thread_n = options.scan_thread_n;
    let get_thread_n = options.get_thread_n;
    let bustub_page_cnt = options.db_size;

    // Create temp dir here, so it will be cleaned when the value is dropped
    let temp_dir = match options.disk_manager_specific.clone() {
        DiskManagerImplementationOptions::Default(d) => if d.file_path.is_some() { None } else { Some(get_tmp_dir()) },
        (_) => None
    };

    let (page_ids, bpm) = init_buffer_pool_manager_for_test(&options, temp_dir);

    let bpm = Arc::new(bpm);
    let page_ids = Arc::new(RwLock::new(page_ids));

    println!("[info] start");

    let mut join_handles: Vec<JoinHandle<()>> = vec![];
    type ModifyRecord = HashMap<PageId, u64>;

    for thread_id in 0..scan_thread_n {
        let bpm = Arc::clone(&bpm);

        let page_ids = Arc::clone(&page_ids);

        let t = thread::spawn(move || unsafe {
            let mut records: ModifyRecord = HashMap::new();

            let run_timer = RunTimer::new(duration_ms);

            let page_idx_start = (bustub_page_cnt * thread_id / scan_thread_n) as PageId;
            let page_idx_end = (bustub_page_cnt * (thread_id + 1) / scan_thread_n) as PageId;
            let mut page_idx = page_idx_start;

            while !run_timer.should_finish() {
                let page_id = page_ids.read()[page_idx as usize];
                let page = bpm.fetch_page(page_id, AccessType::Scan);
                if page.is_none() {
                    continue;
                }

                let page = page.unwrap();

                page.with_write(|u| {
                    if !records.contains_key(&page_idx) {
                        records.insert(page_idx, 0);
                    }

                    let mut seed = records.get_mut(&page_idx).expect("Must exists");

                    check_page_consistent(u.get_data(), page_idx as usize, *seed);
                    *seed = *seed + 1;
                    modify_page(u.get_data_mut(), page_idx as usize, *seed);
                });

                bpm.unpin_page(page_id, true, AccessType::Scan);
                page_idx += 1;
                if page_idx >= page_idx_end {
                    page_idx = page_idx_start;
                }
            }
        });
        join_handles.push(t);
    }

    for _ in 0..get_thread_n {
        let bpm = Arc::clone(&bpm);
        let page_ids = Arc::clone(&page_ids);

        let t = thread::spawn(move || unsafe {
            let mut rng = rand::thread_rng();
            let dist = zipf::ZipfDistribution::new(bustub_page_cnt - 1, 0.8).unwrap();

            let run_timer = RunTimer::new(duration_ms);

            while !run_timer.should_finish() {
                let page_idx = dist.sample(&mut rng);
                let page = bpm.fetch_page(page_ids.read()[page_idx], AccessType::Lookup);

                if page.is_none() {
                    eprintln!("cannot fetch page");
                    panic!();
                }

                let page = page.unwrap();

                let mut page_id: PageId = 0;

                page.with_read(|u| {
                    page_id = u.get_page_id();
                    check_page_consistent_no_seed(u.get_data(), page_idx);
                });

                bpm.unpin_page(page_id, false, AccessType::Lookup);
            }
        });
        join_handles.push(t);
    }

    for t in join_handles {
        t.join().unwrap();
    }

    println!("[info] finish");
}

fn init_buffer_pool_manager_for_test(options: &Options, temp_dir: Option<TempDir>) -> (Vec<PageId>, BufferPoolManager) {
    let mut page_ids: Vec<PageId> = vec![];

    let mut bpm_raw: BufferPoolManager;

    // ############### Setup Buffer pool manager ########################
    // with the different impl of disk manager
    // TODO - cleanup if possible
    match options.disk_manager_specific.clone() {
        DiskManagerImplementationOptions::Default(o) => unsafe {
            let file_path = o.file_path.unwrap_or_else(|| {
                temp_dir.unwrap().into_path().join("data.db")
            });
            bpm_raw = create_bpm(
                &options,
                Arc::new(Mutex::new(DefaultDiskManager::new(file_path).expect("Must be able to create disk manager"))),
            );

            initialize_bpm_pages(&options, &mut page_ids, &mut bpm_raw);
        }
        DiskManagerImplementationOptions::UnlimitedMemory(o) => unsafe {
            let manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
            let cloned_manager = Arc::clone(&manager);

            bpm_raw = create_bpm(&options, manager);

            initialize_bpm_pages(&options, &mut page_ids, &mut bpm_raw);

            // enable disk latency after creating all pages if enabled and if matching the disk manager implementation
            cloned_manager.lock().enable_latency_simulator(o.enable_latency);
        }
    }

    (page_ids, bpm_raw)
}

unsafe fn initialize_bpm_pages(options: &Options, page_ids: &mut Vec<PageId>, bpm: &mut BufferPoolManager) {
    for i in 0..options.db_size {
        let page = bpm.new_page().expect("Must be able to create a page");
        let page_id = page.with_read(|u| u.get_page_id());

        page.with_write(|u| unsafe {
            modify_page(u.get_data_mut(), i, 0);
        });

        bpm.unpin_page(page_id, true, AccessType::default());

        page_ids.push(page_id);
    }
}

fn create_bpm(options: &Options, m: Arc<Mutex<(impl DiskManager + 'static)>>) -> BufferPoolManager {
    BufferPoolManager::new(
        options.bpm_size,
        m,
        Some(options.lru_k_size),
        None, /* log manager */
    )
}

use crate::buffer_pool_manager::multi_threads_tests::options::{OptionsBuilder, UnlimitedMemoryDiskManagerOptions};


// ########################
//     Unlimited Memory
// ########################

#[test]
fn multi_threaded_memory_disk_manager() {
    let options = OptionsBuilder::default()
        .disk_manager_specific(
            DiskManagerImplementationOptions::UnlimitedMemory(
                UnlimitedMemoryDiskManagerOptions {
                    enable_latency: false
                }
            )
        )
        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_5s() {
    let options = OptionsBuilder::default()
        // 5s
        .duration_ms(5000)
        .disk_manager_specific(
            DiskManagerImplementationOptions::UnlimitedMemory(
                UnlimitedMemoryDiskManagerOptions {
                    enable_latency: false
                }
            )
        )
        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_with_latency() {
    let options = OptionsBuilder::default()
        // 4s
        .duration_ms(4000)

        .disk_manager_specific(
            DiskManagerImplementationOptions::UnlimitedMemory(
                UnlimitedMemoryDiskManagerOptions {
                    enable_latency: true
                }
            )
        )
        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_1_scan_and_1_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(1)
        .get_thread_n(1)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_1_scan_and_2_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(1)
        .get_thread_n(2)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_1_scan_and_10_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(1)
        .get_thread_n(10)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_2_scan_and_1_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(2)
        .get_thread_n(1)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_10_scan_and_1_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(10)
        .get_thread_n(1)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_2_scan_and_2_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(2)
        .get_thread_n(2)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_memory_disk_manager_10_scan_and_10_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(10)
        .get_thread_n(10)

        .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

// ########################
//         Default
// ########################

#[test]
fn multi_threaded_default_disk_manager() {
    let options = OptionsBuilder::default()
        .disk_manager_specific(
            DiskManagerImplementationOptions::get_default()
        )
        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_5s() {
    let options = OptionsBuilder::default()

        // 5s
        .duration_ms(5000)

        .disk_manager_specific(
            DiskManagerImplementationOptions::get_default()
        )
        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_1_scan_and_1_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(1)
        .get_thread_n(1)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_1_scan_and_2_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(1)
        .get_thread_n(2)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_1_scan_and_10_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(1)
        .get_thread_n(10)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_2_scan_and_1_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(2)
        .get_thread_n(1)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_10_scan_and_1_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(10)
        .get_thread_n(1)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_2_scan_and_2_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(2)
        .get_thread_n(2)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

#[test]
fn multi_threaded_default_disk_manager_10_scan_and_10_get_threads() {
    let options = OptionsBuilder::default()
        .scan_thread_n(10)
        .get_thread_n(10)

        .disk_manager_specific(DiskManagerImplementationOptions::get_default())

        .build()
        .unwrap();

    run_multi_threads_tests(options)
}

