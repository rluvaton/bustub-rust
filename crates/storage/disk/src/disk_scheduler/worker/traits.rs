use std::sync::Arc;

use crate::{DiskManager, DiskRequestType};

pub(crate) enum DiskSchedulerWorkerMessage<'a> {
    Terminate,
    NewJob(DiskRequestType<'a>),
}

pub trait DiskSchedulerWorker: 'static + Sync + Send {
    fn new<D: DiskManager + 'static>(disk_manager: Arc<D>) -> Self where Self: Sized;

    fn send(&self, message: DiskSchedulerWorkerMessage<'static>);
    
    fn stop(&mut self);
    
    fn boxed(self) -> Box<dyn DiskSchedulerWorker> where Self: Sized {
        Box::new(self)
    }
}

