use crate::disk_scheduler::worker::traits::{DiskSchedulerWorker, DiskSchedulerWorkerMessage};
use crate::{DiskManager, DiskRequestType};
use common::{abort_process_on_panic, Channel};
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

pub(crate) struct DifferentThreadDiskSchedulerWorker {
    thread: Option<JoinHandle<()>>,

    /** A shared queue to concurrently schedule and process requests. When the DiskScheduler's destructor is called,
                    * `std::nullopt` is put into the queue to signal to the background thread to stop execution. */
    sender: Arc<Channel<DiskSchedulerWorkerMessage<'static>>>,
}


// Influenced from
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch20-06-graceful-shutdown-and-cleanup.html
impl DiskSchedulerWorker for DifferentThreadDiskSchedulerWorker {
    /**
     * TODO(P1): Add implementation
     *
     * @brief Background worker thread function that processes scheduled requests.
     *
     * The background thread needs to process requests while the DiskScheduler exists, i.e., this function should not
     * return until ~DiskScheduler() is called. At that point you need to make sure that the function does return.
     */
    fn new<D: DiskManager + 'static>(disk_manager: Arc<D>) -> Self  {

        // let (sender, receiver) = mpsc::channel();

        let channel = Arc::new(Channel::new());

        
        let receiver = channel.clone();
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
                            disk_manager.read_page(req.page_id, req.data.as_mut_slice());
                            req.callback.set_value(true);
                        }
                        DiskRequestType::Write(req) => {
                            disk_manager.write_page(req.page_id, req.data.as_slice());
                            req.callback.set_value(true);
                        }
                        DiskRequestType::WriteAndRead(req) => {
                            disk_manager.write_page(req.dest_page_id, req.data.as_slice());

                            // TODO - read page if write was successful
                            //        as otherwise, if the write failed we will read and lose the data
                            disk_manager.read_page(req.source_page_id, req.data.as_mut_slice());
                            req.callback.set_value(true);
                        }
                    }
                }
            })
            .expect("failed to spawn disk scheduler thread");

        DifferentThreadDiskSchedulerWorker {
            thread: Some(thread),
            sender: channel,
        }
    }

    fn send(&self, message: DiskSchedulerWorkerMessage<'static>) {
        self.sender.put(message);
    }

    fn stop(&mut self) {
        self.sender.put(DiskSchedulerWorkerMessage::Terminate);

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}

