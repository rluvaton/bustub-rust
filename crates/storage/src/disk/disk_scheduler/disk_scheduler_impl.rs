use crate::disk::disk_manager::DiskManager;
use crate::disk::disk_scheduler::disk_request::DiskRequestType;
use crate::disk::disk_scheduler::disk_scheduler::{DiskScheduler, DiskSchedulerWorker, DiskSchedulerWorkerMessage};
use common::Promise;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use parking_lot::Mutex;
use std::thread;

type DiskSchedulerPromise = Promise<bool>;

impl<const Size: usize> DiskScheduler<Size> {
    pub fn new(disk_manager: Arc<Mutex<(impl DiskManager + Send + 'static)>>) -> Self {
        // TODO(P1): remove this line after you have implemented the disk scheduler API
        // unimplemented!(
        //     "DiskScheduler is not implemented yet. If you have finished implementing the disk scheduler, please remove the throw exception line in `disk_scheduler.cpp`.");

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut scheduler = DiskScheduler {
            // disk_manager: Arc::new(Mutex::new(disk_manager)),
            worker: DiskSchedulerWorker::new(disk_manager, Arc::clone(&receiver)),
            sender,
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
    pub fn schedule(&mut self, r: DiskRequestType<Size>) {
        // Schedules a request for the `DiskManager` to execute.
        // The `DiskRequest` struct specifies whether the request is for a read/write, where the data should be written into/from, and the page ID for the operation.
        // The `DiskRequest` also includes a `std::promise` whose value should be set to true once the request is processed.
        //
        self.sender.send(DiskSchedulerWorkerMessage::NewJob(r)).unwrap()
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

impl<const Size: usize> Drop for DiskScheduler<Size> {
    fn drop(&mut self) {
        self.sender.send(DiskSchedulerWorkerMessage::Terminate).unwrap();

        if let Some(thread) = self.worker.thread.take() {
            thread.join().unwrap();
        }
    }
}

// Influenced from
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch20-06-graceful-shutdown-and-cleanup.html
impl<const Size: usize> DiskSchedulerWorker<Size> {
    /**
     * TODO(P1): Add implementation
     *
     * @brief Background worker thread function that processes scheduled requests.
     *
     * The background thread needs to process requests while the DiskScheduler exists, i.e., this function should not
     * return until ~DiskScheduler() is called. At that point you need to make sure that the function does return.
     */
    fn new(disk_manager: Arc<Mutex<(impl DiskManager + Send + 'static)>>, receiver: Arc<Mutex<Receiver<DiskSchedulerWorkerMessage<Size>>>>) -> DiskSchedulerWorker<Size> {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().recv().unwrap();

                let mut manager = disk_manager.lock();

                let req: DiskRequestType<Size>;

                match job {
                    DiskSchedulerWorkerMessage::Terminate => {
                        break
                    }
                    DiskSchedulerWorkerMessage::NewJob(job) => {
                        req = job;
                    }
                }

                match req {
                    DiskRequestType::Read(mut req) => {
                        manager.read_page(req.page_id, req.data.clone().lock().as_mut());
                        req.callback.set_value(true);
                    }
                    DiskRequestType::Write(req) => {
                        manager.write_page(req.page_id, req.data.clone().as_ref());
                        req.callback.set_value(true);
                    }
                }
            }
        });

        DiskSchedulerWorker {
            thread: Some(thread),
        }
    }
}

