use std::cmp::max;
use super::helpers::get_tmp_dir;
use super::options::{DiskManagerImplementationOptions, DurationType, Options};
use crate::buffer::{AccessType, BufferPool, BufferPoolManager};
use pages::{PageData, PageId};
use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use tempdir::TempDir;
use disk_storage::{DefaultDiskManager, DiskManager, DiskManagerUnlimitedMemory};

// This is the structure of the page
#[allow(unused)]
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
fn check_page_consistent_no_seed(data: &PageData, page_idx: PageId) {
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

fn check_page_consistent(data: &PageData, page_idx: PageId, seed: u64) {
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

fn run_multi_threaded_tests_with_timeout(options: Options) {
    let get_thread_duration_type = options.get_thread_duration_type.clone();
    let scan_thread_duration_type = options.scan_thread_duration_type.clone();

    let one_minute = Duration::from_secs(60).as_millis() as u64;

    let mut timeout_in_ms = match (get_thread_duration_type, scan_thread_duration_type) {
        (DurationType::TimeAsMilliseconds(g), DurationType::TimeAsMilliseconds(s)) => max(g, s) + 3_000,

        (DurationType::TimeAsMilliseconds(ms), DurationType::Iteration(_)) |
        (DurationType::Iteration(_), DurationType::TimeAsMilliseconds(ms)) => max(ms, one_minute) + 3_000,
        (_, _) => one_minute
    };


    // if using file system, increase timeout to 1m
    match options.disk_manager_specific {
        DiskManagerImplementationOptions::Default(_) => {
            if timeout_in_ms < one_minute {
                timeout_in_ms = one_minute;
            }
        }
        _ => {}
    }

    let lock = Arc::new(Mutex::new(()));

    let guard = lock.lock();

    let lock = Arc::clone(&lock);
    // This will fail if was unable to finish before the timeout passed
    let timeout_thread = thread::spawn(move || {
        let timeout = Duration::from_millis(timeout_in_ms);
        let res = lock.try_lock_for(timeout);

        if res.is_none() {
            panic!("Thread reached timeout {:?}", timeout);
        }
    });

    run_multi_threads_tests(options);

    drop(guard);

    timeout_thread.join().unwrap();
}

fn run_multi_threads_tests(options: Options) {
    // let duration_ms = options.duration_ms;
    let scan_thread_n = options.scan_thread_n;
    let get_thread_n = options.get_thread_n;
    let bustub_page_cnt = options.db_size;
    let get_thread_page_id_type = options.get_thread_page_id_type.clone();
    let get_thread_duration_type = options.get_thread_duration_type.clone();
    let scan_thread_duration_type = options.scan_thread_duration_type.clone();

    // Create temp dir here, so it will be cleaned when the value is dropped
    let temp_dir = match options.disk_manager_specific.clone() {
        DiskManagerImplementationOptions::Default(d) => if d.file_path.is_some() { None } else { Some(get_tmp_dir()) },
        _ => None
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
        let duration_type = scan_thread_duration_type.clone();

        let t = thread::spawn(move || {
            let mut records: ModifyRecord = HashMap::new();

            let mut duration_runner = duration_type.create_runner();

            let page_idx_start = (bustub_page_cnt * thread_id / scan_thread_n) as PageId;
            let page_idx_end = (bustub_page_cnt * (thread_id + 1) / scan_thread_n) as PageId;
            let mut page_idx = page_idx_start;

            while !duration_runner.should_finish() {
                let page_id = page_ids.read()[page_idx as usize];
                let page = bpm.fetch_page_write(page_id, AccessType::Scan);
                if page.is_err() {
                    continue;
                }

                let mut page = page.unwrap();

                if !records.contains_key(&page_idx) {
                    records.insert(page_idx, 0);
                }

                let seed = records.get_mut(&page_idx).expect("Must exists");

                check_page_consistent(page.get_data(), page_idx, *seed);
                *seed = *seed + 1;
                modify_page(page.get_data_mut(), page_idx as usize, *seed);

                drop(page);
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
        let get_thread_page_id_type = get_thread_page_id_type.clone();
        let duration_type = get_thread_duration_type.clone();

        let t = thread::spawn(move || {
            let mut page_id_getter = get_thread_page_id_type.create_getter(0, (bustub_page_cnt - 1) as PageId);

            let mut duration_runner = duration_type.create_runner();

            while !duration_runner.should_finish() {
                let page_idx = page_id_getter.get();
                let page = bpm.fetch_page_read(page_ids.read()[page_idx as usize], AccessType::Lookup);

                match page {
                    Ok(page) => {
                        check_page_consistent_no_seed(page.get_data(), page_idx);
                    }
                    Err(err) => {
                        eprintln!("cannot fetch page {}", err);
                        panic!();
                    }
                }
            }
        });
        join_handles.push(t);
    }

    for t in join_handles {
        t.join().unwrap();
    }

    println!("[info] finish");
}

fn init_buffer_pool_manager_for_test(options: &Options, temp_dir: Option<TempDir>) -> (Vec<PageId>, Arc<BufferPoolManager>) {
    let mut page_ids: Vec<PageId> = vec![];

    let mut bpm_raw: Arc<BufferPoolManager>;

    // ############### Setup Buffer pool manager ########################
    // with the different impl of disk manager
    // TODO - cleanup if possible
    match options.disk_manager_specific.clone() {
        DiskManagerImplementationOptions::Default(o) => {
            let file_path = o.file_path.unwrap_or_else(|| {
                temp_dir.unwrap().into_path().join("data.db")
            });
            bpm_raw = create_bpm(
                &options,
                Arc::new(Mutex::new(DefaultDiskManager::new(file_path).expect("Must be able to create disk manager"))),
            );

            initialize_bpm_pages(&options, &mut page_ids, bpm_raw.clone());
        }
        DiskManagerImplementationOptions::UnlimitedMemory(o) => {
            let manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
            let cloned_manager = Arc::clone(&manager);

            bpm_raw = create_bpm(&options, manager);

            initialize_bpm_pages(&options, &mut page_ids, bpm_raw.clone());

            // enable disk latency after creating all pages if enabled and if matching the disk manager implementation
            cloned_manager.lock().enable_latency_simulator(o.enable_latency);
        }
    }

    (page_ids, bpm_raw)
}

fn initialize_bpm_pages(options: &Options, page_ids: &mut Vec<PageId>, bpm: Arc<BufferPoolManager>) {
    for i in 0..options.db_size {
        let mut page = bpm.new_page(AccessType::Unknown).expect("Must be able to create a page");

        modify_page(page.get_data_mut(), i, 0);

        page_ids.push(page.get_page_id());
    }
}

fn create_bpm(options: &Options, m: Arc<Mutex<(impl DiskManager + 'static)>>) -> Arc<BufferPoolManager> {
    BufferPoolManager::new(
        options.bpm_size,
        m,
        Some(options.lru_k_size),
        None, /* log manager */
    )
}

#[cfg(test)]
mod tests {
    use super::super::options::*;
    use super::*;


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

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_5s() {
        let options = OptionsBuilder::default()
            // 5s
            .scan_thread_duration_type(DurationType::TimeAsMilliseconds(5000))
            .get_thread_duration_type(DurationType::TimeAsMilliseconds(5000))
            .disk_manager_specific(
                DiskManagerImplementationOptions::UnlimitedMemory(
                    UnlimitedMemoryDiskManagerOptions {
                        enable_latency: false
                    }
                )
            )
            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_with_latency() {
        let options = OptionsBuilder::default()
            // 4s
            .scan_thread_duration_type(DurationType::TimeAsMilliseconds(4000))
            .get_thread_duration_type(DurationType::TimeAsMilliseconds(4000))

            .disk_manager_specific(
                DiskManagerImplementationOptions::UnlimitedMemory(
                    UnlimitedMemoryDiskManagerOptions {
                        enable_latency: true
                    }
                )
            )
            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_0_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(0)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_0_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(0)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_1_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_2_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_10_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_2_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_memory_disk_manager_10_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_unlimited_memory())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
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

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_5s() {
        let options = OptionsBuilder::default()

            // 5s
            .scan_thread_duration_type(DurationType::TimeAsMilliseconds(5000))
            .get_thread_duration_type(DurationType::TimeAsMilliseconds(5000))

            .disk_manager_specific(
                DiskManagerImplementationOptions::get_default()
            )
            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_0_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(0)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_0_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(0)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_1_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(1)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_2_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_10_scan_and_1_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(1)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_2_scan_and_2_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(2)
            .get_thread_n(2)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }

    #[test]
    fn multi_threaded_default_disk_manager_10_scan_and_10_get_threads() {
        let options = OptionsBuilder::default()
            .scan_thread_n(10)
            .get_thread_n(10)

            .disk_manager_specific(DiskManagerImplementationOptions::get_default())

            .build()
            .unwrap();

        run_multi_threaded_tests_with_timeout(options)
    }
}
