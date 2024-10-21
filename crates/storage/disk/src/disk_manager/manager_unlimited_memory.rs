use crate::DiskManager;
use common::Future;
use pages::{PageData, PageId, PAGE_SIZE};
use parking_lot::{Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::{sleep, ThreadId};
use std::time::Duration;

type Page = PageData;
type ProtectedPage = Arc<Mutex<Option<Page>>>;

fn create_page() -> Page {
    [0u8; PAGE_SIZE]
}

struct LatencyProcessor {
    recent_access: [PageId; 4],
    access_ptr: u64,
}

struct DiskManagerUnlimitedMemoryData {
    // Thread id change with the data itself
    thread_id: Option<ThreadId>,

    // Vector of protected pages
    // each protected page has a lock to the underlying value
    pages: Vec<ProtectedPage>,
}


/**
 * DiskManager takes care of the allocation and deallocation of pages within a database. It performs the reading and
 * writing of pages to and from disk, providing a logical file layer within the context of a database management system.
 */
pub struct DiskManagerUnlimitedMemory {
    latency_simulator_enabled: AtomicBool,

    // Access together so grouped together
    latency_processor_mutex: Mutex<LatencyProcessor>,

    // Access the data using the lock for thread safety
    data: Mutex<DiskManagerUnlimitedMemoryData>,
}

unsafe impl Send for DiskManagerUnlimitedMemory {}


impl DiskManagerUnlimitedMemory {
    /**
     * Creates a new disk manager that writes to the specified database file.
     * @param db_file the file name of the database file to write to
     */
    pub fn new() -> DiskManagerUnlimitedMemory {
        DiskManagerUnlimitedMemory {
            latency_simulator_enabled: AtomicBool::new(false),

            latency_processor_mutex: Mutex::new(LatencyProcessor {
                recent_access: [0; 4],
                access_ptr: 0,
            }),

            data: Mutex::new(
                DiskManagerUnlimitedMemoryData {
                    pages: vec![],
                    thread_id: None,
                }
            ),
        }
    }

    fn process_latency(&self, page_id: PageId) {
        if !self.latency_simulator_enabled.load(Ordering::Relaxed) {
            return;
        }

        let mut sleep_micro_sec = 1000u64;  // for random access, 1ms latency
        {
            // std::unique_lock<std::mutex> lck(latency_processor_mutex_);
            let latency_processor = self.latency_processor_mutex.lock();
            for recent_page_id in latency_processor.recent_access {
                if recent_page_id & (!0x3) == page_id & (!0x3) {
                    sleep_micro_sec = 100;  // for access in the same "block", 0.1ms latency
                    break;
                }

                if page_id >= recent_page_id && page_id <= recent_page_id + 3 {
                    sleep_micro_sec = 100;  // for sequential access, 0.1ms latency
                    break;
                }
            }
        }
        sleep(Duration::from_micros(sleep_micro_sec));
    }

    fn post_process_latency(&self, page_id: PageId) {
        if !self.latency_simulator_enabled.load(Ordering::Relaxed) {
            return;
        }

        // std::scoped_lock<std::mutex> lck(latency_processor_mutex_);
        let mut latency_processor = self.latency_processor_mutex.lock();
        let access_ptr = latency_processor.access_ptr;
        latency_processor.recent_access[access_ptr as usize] = page_id;
        latency_processor.access_ptr = (access_ptr + 1) % latency_processor.recent_access.len() as u64;
    }

    pub fn enable_latency_simulator(&self, enabled: bool) {
        self.latency_simulator_enabled.store(enabled, Ordering::SeqCst);
    }

    #[allow(unused)]
    fn get_last_read_thread_and_clear(&mut self) -> Option<ThreadId> {
        let mut lock = self.data.lock();
        let t = lock.thread_id;

        lock.thread_id = None;

        t
    }
}

impl DiskManager for DiskManagerUnlimitedMemory {
    fn shut_down(&mut self) {}

    /**
     * Write a page to the database file.
     * @param page_id id of the page
     * @param page_data raw page data
     */
    fn write_page(&self, page_id: PageId, page_data: &[u8]) {
            self.process_latency(page_id);

            let page_ref: Arc<Mutex<Option<Page>>>;
            let mut page_lock: MutexGuard<Option<Page>>;

            {
                let mut data = self.data.lock();

                if data.thread_id.is_none() {
                    data.thread_id = Some(thread::current().id());
                }

                if page_id >= data.pages.len() as i32 {
                    data.pages.resize_with((page_id + 1) as usize, || Arc::new(Mutex::new(None)));
                }

                page_ref = Arc::clone(&data.pages[page_id as usize]);

                page_lock = page_ref.lock()

                // Unlock the page table lock
            }

            if page_lock.is_none() {
                *page_lock = Some(create_page());
            }


            // Get the value from the page
            let mut value = (*page_lock).unwrap();

            // Using clone to avoid data being modified when using that page_data reference
            value[0..PAGE_SIZE].clone_from_slice(page_data);

            // Set it back
            *page_lock = Some(value);

            self.post_process_latency(page_id);

            // Unlock the single page lock
    }

    /**
     * Read a page from the database file.
     * @param page_id id of the page
     * @param[out] page_data output buffer
     */
    fn read_page(&self, page_id: PageId, page_data: &mut [u8]) {
        self.process_latency(page_id);

        let page_ref: Arc<Mutex<Option<Page>>>;
        let page_lock: MutexGuard<Option<Page>>;

        {
            let mut data = self.data.lock();

            if data.thread_id.is_none() {
                data.thread_id = Some(thread::current().id());
            }

            if page_id >= data.pages.len() as i32 {
                panic!("page {} not in range", page_id);
            }

            page_ref = Arc::clone(&data.pages[page_id as usize]);
            page_lock = page_ref.lock()

            // Page table lock dropped
        }

        if page_lock.is_none() {
            panic!("page {} not exists", page_id);
        }

        page_data[0..PAGE_SIZE].copy_from_slice(&page_lock.unwrap());

        self.post_process_latency(page_id);

        // Single page lock dropped
    }

    fn write_log(&self, _log_data: &[u8], _size: i32) {
        unimplemented!();
    }

    fn read_log(&self, _log_data: &mut [u8], _size: i32, _offset: i32) -> bool {
        unimplemented!();
    }

    fn get_num_flushes(&self) -> i32 {
        unimplemented!();
    }

    fn get_flush_state(&self) -> bool {
        unimplemented!();
    }

    fn get_num_writes(&self) -> i32 {
        unimplemented!();
    }

    fn set_flush_log_future(&mut self, _f: Option<Future<()>>) {
        unimplemented!();
    }

    fn has_flush_log_future(&self) -> bool {
        unimplemented!();
    }
}

impl Default for DiskManagerUnlimitedMemory {
    fn default() -> Self {
        Self::new()
    }
}