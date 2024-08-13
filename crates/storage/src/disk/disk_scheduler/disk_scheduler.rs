use std::thread::Thread;
use crate::disk::disk_manager::DiskManager;

/**
 * @brief The DiskScheduler schedules disk read and write operations.
 *
 * A request is scheduled by calling DiskScheduler::Schedule() with an appropriate DiskRequest object. The scheduler
 * maintains a background worker thread that processes the scheduled requests using the disk manager. The background
 * thread is created in the DiskScheduler constructor and joined in its destructor.
 */
pub struct DiskScheduler {

    /** Pointer to the disk manager. */
    // DiskManager *disk_manager_ __attribute__((__unused__));
    disk_manager: Box<dyn DiskManager>,

    /** A shared queue to concurrently schedule and process requests. When the DiskScheduler's destructor is called,
        * `std::nullopt` is put into the queue to signal to the background thread to stop execution. */
    // Channel<std::optional<DiskRequest>> request_queue_;
    // TODO ----------- FIX TYPE -------
    request_queue: Option<()>,

    /** The background thread responsible for issuing scheduled requests to the disk manager. */
    // std::optional<std::thread> background_thread_;
    background_thread: Option<Thread>,
}
