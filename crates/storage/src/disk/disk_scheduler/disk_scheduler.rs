use std::sync::Arc;
use crate::disk::disk_scheduler::disk_request::DiskRequestType;
use common::Channel;

use concurrency_shared::{
   thread::JoinHandle
};

/**
 * @brief The DiskScheduler schedules disk read and write operations.
 *
 * A request is scheduled by calling DiskScheduler::Schedule() with an appropriate DiskRequest object. The scheduler
 * maintains a background worker thread that processes the scheduled requests using the disk manager. The background
 * thread is created in the DiskScheduler constructor and joined in its destructor.
 */
pub struct DiskScheduler {
    /** The background thread responsible for issuing scheduled requests to the disk manager. */
    pub(crate) worker: DiskSchedulerWorker,

    /** A shared queue to concurrently schedule and process requests. When the DiskScheduler's destructor is called,
           * `std::nullopt` is put into the queue to signal to the background thread to stop execution. */
    pub(crate) sender: Arc<Channel<DiskSchedulerWorkerMessage>>,
}


pub(crate) enum DiskSchedulerWorkerMessage {
    Terminate,
    NewJob(DiskRequestType)
}


pub(crate) struct DiskSchedulerWorker {
    pub(crate) thread: Option<JoinHandle<()>>,
}
