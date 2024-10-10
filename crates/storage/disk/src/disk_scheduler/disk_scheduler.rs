use parking_lot::{Mutex, MutexGuard};
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

use crate::disk_scheduler::disk_request::WriteAndReadDiskRequest;
use crate::{DiskManager, DiskRequestType, ReadDiskRequest, WriteDiskRequest};
use common::{abort_process_on_panic, Channel, Future, FutureLifetime, Promise, PromiseLifetime};
use pages::{PageData, PageId, PageWriteGuard, UnderlyingPage};

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
    sender: Arc<Channel<DiskSchedulerWorkerMessage<'static>>>,
}


enum DiskSchedulerWorkerMessage<'a> {
    Terminate,
    NewJob(DiskRequestType<'a>),
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

    /// Schedule read page from disk
    ///
    /// this is not blocking
    ///
    /// # Safety
    /// You must not drop the page before calling `.wait` on the result as it will cause undefined behavior
    ///
    /// this only change the page data and nothing more
    ///
    #[must_use]
    pub unsafe fn schedule_read_page_from_disk(&mut self, dest: &mut UnderlyingPage) -> Future<bool> {
        // promise value should be set to true once the request is processed.
        let promise = Promise::new();
        let future = promise.get_future();

        // change dest lifetime
        let dest_updated_lifetime: &'static mut PageData = unsafe {
            let ptr: *mut PageData = dest.get_data_mut();

            &mut *ptr
        };

        let request = ReadDiskRequest::new(dest.get_page_id(), dest_updated_lifetime, promise);

        self.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }

    /// Read page from disk
    ///
    /// This block until the page is read after the provided callback is called
    ///
    /// this only change the page data and nothing more
    ///
    pub fn read_page_from_disk<'a, R, AfterRequestFn: FnOnce(MutexGuard<Self>) -> R>(scheduler: MutexGuard<Self>, dest: &mut UnderlyingPage, after_request_fn: AfterRequestFn) -> (bool, R) {
        // promise value should be set to true once the request is processed.
        let promise = Promise::new();
        let future = promise.get_future();

        // Change data lifetime
        //
        // SAFETY: this is safe as we wait until the scheduler request finish,
        // and we hold mutable reference to the underlying page
        // so the compiler will disallow other references as well
        // so nothing should read the page in the middle (while reading from disk)
        let dest_updated_lifetime: &'static mut PageData = unsafe {
            let ptr: *mut PageData = dest.get_data_mut();

            &mut *ptr
        };

        let request = ReadDiskRequest::new(dest.get_page_id(), dest_updated_lifetime, promise);

        scheduler.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        let r = after_request_fn(scheduler);

        (future.wait(), r)
    }

    /// Schedule page to be written to disk
    /// this is not blocking
    ///
    /// # Safety
    /// You must not drop the page before calling `.wait` on the result as it will cause undefined behavior
    ///
    #[must_use]
    pub unsafe fn schedule_write_page_to_disk<'a>(&mut self, page_to_write: &UnderlyingPage) -> Future<bool> {
        // promise value should be set to true once the request is processed.

        let promise = Promise::new();
        let future = promise.get_future();

        // change src lifetime
        let src_updated_lifetime: &'static PageData = unsafe {
            let ptr: *const PageData = page_to_write.get_data();

            &*ptr
        };

        let request = WriteDiskRequest::new(page_to_write.get_page_id(), src_updated_lifetime, promise);

        self.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }

    /// Write page to disk
    ///
    /// This block until the page is written after the provided callback is called
    pub fn write_page_to_disk<'a, R, AfterRequestFn: FnOnce(MutexGuard<Self>) -> R>(scheduler: MutexGuard<Self>, page_to_write: &UnderlyingPage, after_request_fn: AfterRequestFn) -> (bool, R) {
        // promise value should be set to true once the request is processed.

        let promise = Promise::new();
        let future = promise.get_future();

        // Change data lifetime
        //
        // SAFETY: this is safe as we wait until the scheduler request finish,
        // and we hold a shared reference to the underlying page
        // so the compiler will disallow writers to override the page in the middle
        let src_updated_lifetime: &'static PageData = unsafe {
            let ptr: *const PageData = page_to_write.get_data();

            &*ptr
        };

        let request = WriteDiskRequest::new(page_to_write.get_page_id(), src_updated_lifetime, promise);

        scheduler.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        let r = after_request_fn(scheduler);

        (future.wait(), r)
    }

    /// Write page and then read a new page from disk
    ///
    /// This block until the page is read after the provided callback is called
    ///
    /// this only change the page data and nothing more
    pub fn write_and_read_page_from_disk<'a, R, AfterRequestFn: FnOnce(MutexGuard<Self>) -> R>(scheduler: MutexGuard<Self>, page: &mut UnderlyingPage, page_id_to_read: PageId, after_request_fn: AfterRequestFn) -> (bool, R) {
        // promise value should be set to true once the request is processed.
        let promise = Promise::new();
        let future = promise.get_future();

        // Change data lifetime
        //
        // SAFETY: this is safe as we wait until the scheduler request finish,
        // and we hold mutable reference to the underlying page
        // so the compiler will disallow other mutable references as well
        // so nothing should change the page in the middle (while writing page to disk) or read the page in the middle (while reading from disk)
        let data_updated_lifetime: &'static mut PageData = unsafe {
            let ptr: *mut PageData = page.get_data_mut();

            &mut *ptr
        };

        let request = WriteAndReadDiskRequest::new(page.get_page_id(), page_id_to_read, data_updated_lifetime, promise);

        scheduler.sender.put(DiskSchedulerWorkerMessage::NewJob(request.into()));

        let r = after_request_fn(scheduler);

        (future.wait(), r)
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
    fn new<D: DiskManager + Send + Sync + 'static>(disk_manager: Arc<Mutex<D>>, receiver: Arc<Channel<DiskSchedulerWorkerMessage<'static>>>) -> DiskSchedulerWorker {
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
                        DiskRequestType::Read(req) => {
                            disk_manager.lock().read_page(req.page_id, req.data.as_mut_slice());
                            req.callback.set_value(true);
                        }
                        DiskRequestType::Write(req) => {
                            disk_manager.lock().write_page(req.page_id, req.data.as_slice());
                            req.callback.set_value(true);
                        }
                        DiskRequestType::WriteAndRead(req) => {
                            disk_manager.lock().write_page(req.dest_page_id, req.data.as_slice());

                            // TODO - read page if write was successful
                            //        as otherwise, if the write failed we will read and lose the data
                            disk_manager.lock().read_page(req.source_page_id, req.data.as_mut_slice());
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

