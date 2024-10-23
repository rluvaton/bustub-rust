use std::sync::Arc;

use crate::disk_scheduler::disk_request::WriteAndReadDiskRequest;
use crate::disk_scheduler::worker::traits::{DiskSchedulerWorker, DiskSchedulerWorkerMessage};
use crate::{DiskManager, ReadDiskRequest, WriteDiskRequest};
use common::{Future, Promise};
use pages::{PageData, PageId, UnderlyingPage};

#[cfg(not(target_arch = "wasm32"))]
use crate::disk_scheduler::worker::different_thread::DifferentThreadDiskSchedulerWorker;

#[cfg(target_arch = "wasm32")]
use crate::disk_scheduler::worker::same_thread::SameThreadDiskScheduler;

/**
 * @brief The DiskScheduler schedules disk read and write operations.
 *
 * A request is scheduled by calling DiskScheduler::Schedule() with an appropriate DiskRequest object. The scheduler
 * maintains a background worker thread that processes the scheduled requests using the disk manager. The background
 * thread is created in the DiskScheduler constructor and joined in its destructor.
 */
pub struct DiskScheduler {
    /** The background thread responsible for issuing scheduled requests to the disk manager. */
    worker: Box<dyn DiskSchedulerWorker>,
}


type DiskSchedulerPromise = Promise<bool>;

impl DiskScheduler {
    
    pub fn new<D: DiskManager + 'static>(disk_manager: Arc<D>) -> Self {
        // let (sender, receiver) = mpsc::channel();

        let scheduler = DiskScheduler {
            #[cfg(not(target_arch = "wasm32"))]
            worker: DifferentThreadDiskSchedulerWorker::new(disk_manager).boxed(),
            #[cfg(target_arch = "wasm32")]
            worker: SameThreadDiskScheduler::new(disk_manager).boxed(),
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

        self.worker.send(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }

    /// Read page from disk
    ///
    /// This block until the page is read after the provided callback is called
    ///
    /// this only change the page data and nothing more
    ///
    pub fn read_page_from_disk<'a, R, AfterRequestFn: FnOnce() -> R>(self: Arc<Self>, dest: &mut UnderlyingPage, after_request_fn: AfterRequestFn) -> (bool, R) {
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

        self.worker.send(DiskSchedulerWorkerMessage::NewJob(request.into()));

        drop(self);
        let r = after_request_fn();

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

        self.worker.send(DiskSchedulerWorkerMessage::NewJob(request.into()));

        future
    }

    /// Write page to disk
    ///
    /// This block until the page is written after the provided callback is called
    pub fn write_page_to_disk<'a, R, AfterRequestFn: FnOnce() -> R>(self: Arc<Self>, page_to_write: &UnderlyingPage, after_request_fn: AfterRequestFn) -> (bool, R) {
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

        self.worker.send(DiskSchedulerWorkerMessage::NewJob(request.into()));

        drop(self);
        let r = after_request_fn();

        (future.wait(), r)
    }

    /// Write page and then read a new page from disk
    ///
    /// This block until the page is read after the provided callback is called
    ///
    /// this only change the page data and nothing more
    pub fn write_and_read_page_from_disk<'a, R, AfterRequestFn: FnOnce() -> R>(self: Arc<Self>, page: &mut UnderlyingPage, page_id_to_read: PageId, after_request_fn: AfterRequestFn) -> (bool, R) {
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

        self.worker.send(DiskSchedulerWorkerMessage::NewJob(request.into()));

        drop(self);

        let r = after_request_fn();

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
        self.worker.stop();
    }
}

