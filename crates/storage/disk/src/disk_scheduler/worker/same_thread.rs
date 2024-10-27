use crate::disk_scheduler::worker::traits::{DiskSchedulerWorker, DiskSchedulerWorkerMessage};
use crate::{DiskManager, DiskRequestType};
use std::sync::Arc;

pub(crate) struct SameThreadDiskScheduler {
    manager: Arc<dyn DiskManager + 'static>,
}

impl SameThreadDiskScheduler {
    fn handle(&self, message: DiskSchedulerWorkerMessage<'static>) {
        let req: DiskRequestType;

        match message {
            DiskSchedulerWorkerMessage::Terminate => {
                return;
            }
            DiskSchedulerWorkerMessage::NewJob(job) => {
                req = job;
            }
        }

        match req {
            DiskRequestType::Read(req) => {
                self.manager.read_page(req.page_id, req.data.as_mut_slice());
                req.callback.set_value(true);
            }
            DiskRequestType::Write(req) => {
                self.manager.write_page(req.page_id, req.data.as_slice());
                req.callback.set_value(true);
            }
            DiskRequestType::WriteAndRead(req) => {
                self.manager.write_page(req.dest_page_id, req.data.as_slice());

                // TODO - read page if write was successful
                //        as otherwise, if the write failed we will read and lose the data
                self.manager.read_page(req.source_page_id, req.data.as_mut_slice());
                req.callback.set_value(true);
            }
        }
    }
}

impl DiskSchedulerWorker for SameThreadDiskScheduler {
    fn new<D: DiskManager + 'static>(disk_manager: Arc<D>) -> Self
    where
        Self: Sized
    {
        Self {
            manager: disk_manager
        }
    }
    

    fn send(&self, message: DiskSchedulerWorkerMessage<'static>) {
        self.handle(message);
    }

    fn stop(&mut self) {
        self.handle(DiskSchedulerWorkerMessage::Terminate)
    }
}
