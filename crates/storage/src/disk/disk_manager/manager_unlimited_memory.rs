use crate::disk::disk_manager::disk_manager_trait::DiskManager;
use common::config::{PageId, BUSTUB_PAGE_SIZE};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, ThreadId};
use std::time::Duration;
use common::Future;

// TODO - Why shared?
static BUFFER_USED: Mutex<Option<Vec<u8>>> = Mutex::new(None);

type Page = [u8; BUSTUB_PAGE_SIZE as usize];
type ProtectedPage = Mutex<Page>;

fn create_new_protected_page() -> ProtectedPage {
    Mutex::new([0u8; BUSTUB_PAGE_SIZE as usize])
}


/**
 * DiskManager takes care of the allocation and deallocation of pages within a database. It performs the reading and
 * writing of pages to and from disk, providing a logical file layer within the context of a database management system.
 */
pub struct DiskManagerUnlimitedMemory {
    // TODO - default false
    latency_simulator_enabled: bool,

    latency_processor_mutex: Mutex<()>,
    recent_access: [PageId; 4],

    // TODO - default 0
    access_ptr: u64,

    mutex: Mutex<()>,
    // std::optional<std::thread::id> thread_id_;
    thread_id: Option<ThreadId>,
    // std::vector<std::shared_ptr<ProtectedPage>> data_;

    // TODO - should option be inside mutex or outside?
    data: Vec<Arc<Option<ProtectedPage>>>,
}

impl DiskManagerUnlimitedMemory {
    /**
     * Creates a new disk manager that writes to the specified database file.
     * @param db_file the file name of the database file to write to
     */
    pub fn new() -> DiskManagerUnlimitedMemory {
        DiskManagerUnlimitedMemory {
            latency_simulator_enabled: false,

            recent_access: [0; 4],
            latency_processor_mutex: Mutex::new(()),
            access_ptr: 0,

            mutex: Mutex::new(()),
            thread_id: None,
            data: vec![],
        }
    }

    fn process_latency(&mut self, page_id: PageId) {
        if !self.latency_simulator_enabled {
            return;
        }

        let mut sleep_micro_sec = 1000u64;  // for random access, 1ms latency
        {
            // std::unique_lock<std::mutex> lck(latency_processor_mutex_);
            let _lock = self.latency_processor_mutex.lock().unwrap();
            for recent_page_id in self.recent_access {
                if (recent_page_id & (!0x3) == page_id & (!0x3)) {
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

    fn post_process_latency(&mut self, page_id: PageId) {
        if !self.latency_simulator_enabled {
            return;
        }

        // std::scoped_lock<std::mutex> lck(latency_processor_mutex_);
        let lock = self.latency_processor_mutex.lock().unwrap();
        self.recent_access[self.access_ptr as usize] = page_id;
        self.access_ptr = (self.access_ptr + 1) % self.recent_access.len() as u64;
    }

    fn enable_latency_simulator(&mut self, enabled: bool) {
        self.latency_simulator_enabled = enabled;
    }

    fn get_last_read_thread_and_clear(&mut self) -> Option<ThreadId> {
        let lock = self.mutex.lock().unwrap();
        let t = self.thread_id;

        self.thread_id = None;

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
    fn write_page(&mut self, page_id: PageId, page_data: &[u8]) {
        self.process_latency(page_id);

        let mut ptr: Arc<Option<ProtectedPage>>;

        {
            let _lock = self.mutex.lock().unwrap();

            if self.thread_id.is_none() {
                self.thread_id = Some(thread::current().id());
            }

            if page_id >= self.data.len() as i32 {
                self.data.resize((page_id + 1) as usize, Arc::new(None))
            }

            if self.data[page_id as usize].is_none() {
                ptr = Arc::new(
                    Some(
                        create_new_protected_page()
                    )
                );


                // self.data[page_id as usize] = ptr.clone();
            } else {
                ptr = Arc::clone(&self.data[page_id as usize]);
            }
        }

        {
            let a = Arc::get_mut(&mut ptr);

            let mut page_lock = a.unwrap().as_ref().unwrap().lock().unwrap();

            page_lock[0..BUSTUB_PAGE_SIZE as usize].copy_from_slice(page_data);
        }
        self.data[page_id as usize] = ptr;

        self.post_process_latency(page_id);
    }

    /**
     * Read a page from the database file.
     * @param page_id id of the page
     * @param[out] page_data output buffer
     */
    fn read_page(&mut self, page_id: PageId, page_data: &mut [u8]) {
        self.process_latency(page_id);

        let mut ptr: &mut Arc<Option<ProtectedPage>>;

        {
            let _lock = self.mutex.lock().unwrap();

            if self.thread_id.is_none() {
                self.thread_id = Some(thread::current().id());
            }

            if page_id >= self.data.len() as i32 {
                // TODO - output to stderr
                println!("page {} not in range", page_id);
                panic!("page {} not in range", page_id);
            }

            if self.data[page_id as usize].is_none() {
                // TODO - output to stderr
                println!("page {} not exists", page_id);
                panic!("page {} not exists", page_id);
            }

            ptr = &mut self.data[page_id as usize];
        }

        {
            let page_lock = Arc::get_mut(&mut ptr).unwrap().as_ref().unwrap().lock().unwrap();

            page_data[0..BUSTUB_PAGE_SIZE as usize].copy_from_slice(page_lock.as_ref());
        }

        self.post_process_latency(page_id);
    }

    fn write_log(&mut self, log_data: &[u8], size: i32) {
        unimplemented!();
    }

    fn read_log(&mut self, log_data: &mut [u8], size: i32, offset: i32) -> bool {
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

    fn set_flush_log_future(&mut self, f: Option<Future<()>>) {
        unimplemented!();
    }

    fn has_flush_log_future(&self) -> bool {
        unimplemented!();
    }
}
