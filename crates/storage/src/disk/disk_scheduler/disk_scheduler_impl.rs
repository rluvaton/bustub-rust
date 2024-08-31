use crate::disk::disk_manager::DiskManager;
use crate::disk::disk_scheduler::disk_request::DiskRequestType;
use crate::disk::disk_scheduler::disk_scheduler::{DiskScheduler, DiskSchedulerWorker, DiskSchedulerWorkerMessage};
use common::{Channel, Promise};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

type DiskSchedulerPromise = Promise<bool>;

impl DiskScheduler {
    pub fn new(disk_manager: Arc<Mutex<impl DiskManager + 'static>>) -> Self {
        // TODO(P1): remove this line after you have implemented the disk scheduler API
        // unimplemented!(
        //     "DiskScheduler is not implemented yet. If you have finished implementing the disk scheduler, please remove the throw exception line in `disk_scheduler.cpp`.");

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
    fn new(disk_manager: Arc<Mutex<dyn DiskManager + Send + Sync>>, receiver: Arc<Channel<DiskSchedulerWorkerMessage>>) -> DiskSchedulerWorker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.get();

                let mut manager = disk_manager.lock();

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
                    DiskRequestType::Read(req) => unsafe {
                        manager.read_page(req.page_id, req.data.clone().get_mut().as_mut_slice());
                        req.callback.set_value(true);
                    }
                    DiskRequestType::Write(req) => unsafe {
                        manager.write_page(req.page_id, req.data.get().as_slice());
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

