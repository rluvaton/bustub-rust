use parking_lot::Mutex;
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

use common::{abort_process_on_panic, Channel, Future, Promise};
use pages::{PageId, PageReadGuard, PageWriteGuard, UnderlyingPage};
use crate::{DiskManager, DiskRequestType, ReadDiskRequest, WriteDiskRequest};

/**
 * @brief The DiskScheduler schedules disk read and write operations.
 *
 * A request is scheduled by calling DiskScheduler::Schedule() with an appropriate DiskRequest object. The scheduler
 * maintains a background worker thread that processes the scheduled requests using the disk manager. The background
 * thread is created in the DiskScheduler constructor and joined in its destructor.
 */
pub struct DiskScheduler {
    /** The background thread responsible for issuing scheduled requests to the disk manager. */
    worker: DiskSchedulerWorker,

    /** A shared queue to concurrently schedule and process requests. When the DiskScheduler's destructor is called,
                 * `std::nullopt` is put into the queue to signal to the background thread to stop execution. */
    sender: Arc<Channel<DiskSchedulerWorkerMessage>>,
}


enum DiskSchedulerWorkerMessage {
    Terminate,
    NewJob(DiskRequestType),
}


struct DiskSchedulerWorker {
    thread: Option<JoinHandle<()>>,
}


type DiskSchedulerPromise = Promise<bool>;

impl DiskScheduler {
    pub fn new<D: DiskManager + 'static>(disk_manager: Arc<Mutex<D>>) -> Self {
        // let (sender, receiver) = mpsc::channel();

        let channel = Arc::new(Channel::new());

        let scheduler = DiskScheduler {
            // disk_manager: Arc::new(Mutex::new(disk_manager)),
            worker: DiskSchedulerWorker::new(disk_manager, Arc::clone(&channel)),
            sender: Arc::clone(&channel),
        };

        scheduler
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Schedules a request for the DiskManager to execute.
     *
     * @param r The request to be scheduled.
     */
    pub fn schedule(&mut self, r: DiskRequestType) {
        // Schedules a request for the `DiskManager` to execute.
        // The `DiskRequest` struct specifies whether the request is for a read/write, where the data should be written into/from, and the page ID for the operation.
        // The `DiskRequest` also includes a `std::promise` whose value should be set to true once the request is processed.
        //
        self.sender.put(DiskSchedulerWorkerMessage::NewJob(r))
    }

    pub fn schedule_read_page_from_disk(&mut self, page_id_to_read: PageId, dest: PageWriteGuard) -> Future<bool> {
        let promise = Promise::new();
        let future = promise.get_future();

        let request = ReadDiskRequest::new(page_id_to_read, dest, promise);

        self.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }

    pub fn schedule_write_page_to_disk(&mut self, page_id_to_write: PageId, src: PageReadGuard) -> Future<bool> {
        let promise = Promise::new();
        let future = promise.get_future();

        let request = WriteDiskRequest::new(page_id_to_write, src, promise);

        self.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }

    pub fn schedule_write_page_and_read_different_page_to_disk(&mut self, page_id_to_write: PageId, page_id_to_read: PageId, src: PageReadGuard) -> Future<bool> {
        let promise = Promise::new();
        let future = promise.get_future();

        let request = WriteDiskRequest::new(page_id_to_write, src, promise);

        self.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }


    /**
     * @brief Create a Promise object. If you want to implement your own version of promise, you can change this function
     * so that our test cases can use your promise implementation.
     *
     * @return std::promise<bool>
     */
    pub fn create_promise(&self) -> DiskSchedulerPromise {
        Promise::new()
    }
}

impl Drop for DiskScheduler {
    fn drop(&mut self) {
        self.sender.put(DiskSchedulerWorkerMessage::Terminate);

        if let Some(thread) = self.worker.thread.take() {
            thread.join().unwrap();
        }
    }
}

// Influenced from
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch20-06-graceful-shutdown-and-cleanup.html
impl DiskSchedulerWorker {
    /**
     * TODO(P1): Add implementation
     *
     * @brief Background worker thread function that processes scheduled requests.
     *
     * The background thread needs to process requests while the DiskScheduler exists, i.e., this function should not
     * return until ~DiskScheduler() is called. At that point you need to make sure that the function does return.
     */
    fn new<D: DiskManager + Send + Sync + 'static>(disk_manager: Arc<Mutex<D>>, receiver: Arc<Channel<DiskSchedulerWorkerMessage>>) -> DiskSchedulerWorker {
        let thread = Builder::new()
            .name("Disk Scheduler".to_string())
            .spawn(move || {
                abort_process_on_panic();

                loop {
                    let job = receiver.get();

                    let req: DiskRequestType;

                    match job {
                        DiskSchedulerWorkerMessage::Terminate => {
                            break
                        }
                        DiskSchedulerWorkerMessage::NewJob(job) => {
                            req = job;
                        }
                    }

                    match req {
                        DiskRequestType::Read(mut req) => unsafe {
                            disk_manager.lock().read_page(req.page_id, req.data.get_mut().as_mut_slice());
                            req.callback.set_value(true);
                        }
                        DiskRequestType::Write(req) => unsafe {
                            disk_manager.lock().write_page(req.page_id, req.data.get().as_slice());
                            req.callback.set_value(true);
                        }
                    }
                }
            })
            .expect("failed to spawn disk scheduler thread");

        DiskSchedulerWorker {
            thread: Some(thread),
        }
    }
}

