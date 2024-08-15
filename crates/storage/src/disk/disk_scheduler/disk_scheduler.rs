use crate::disk::disk_scheduler::disk_request::DiskRequestType;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

/**
 * @brief The DiskScheduler schedules disk read and write operations.
 *
 * A request is scheduled by calling DiskScheduler::Schedule() with an appropriate DiskRequest object. The scheduler
 * maintains a background worker thread that processes the scheduled requests using the disk manager. The background
 * thread is created in the DiskScheduler constructor and joined in its destructor.
 */
pub struct DiskScheduler<const Size: usize> {

    /** Pointer to the disk manager. */
    // DiskManager *disk_manager_ __attribute__((__unused__));
    // pub(crate) disk_manager: Arc<Mutex<&'a mut DefaultDiskManager>>,

    /** A shared queue to concurrently schedule and process requests. When the DiskScheduler's destructor is called,
        * `std::nullopt` is put into the queue to signal to the background thread to stop execution. */
    // Channel<std::optional<DiskRequest>> request_queue_;
    // TODO ----------- FIX TYPE -------
    // pub(crate) request_queue: SyncSender<DiskSchedulerThreadMessage<Size>>,
    //
    // /** The background thread responsible for issuing scheduled requests to the disk manager. */
    // // std::optional<std::thread> background_thread_;
    // // pub(crate) tmp: i32
    // pub(crate) background_thread: JoinHandle<()>,

    pub(crate) worker: DiskSchedulerWorker<Size>,

    pub(crate) sender: Sender<DiskSchedulerWorkerMessage<Size>>,
}


pub(crate) enum DiskSchedulerWorkerMessage<const Size: usize> {
    Terminate,
    NewJob(DiskRequestType<Size>)
}


pub(crate) struct DiskSchedulerWorker<const Size: usize> {
    pub(crate) thread: Option<JoinHandle<()>>,
}
