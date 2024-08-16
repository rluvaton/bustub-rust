mod manager_impl;

use common::config::{AtomicLSN, LOG_BUFFER_SIZE};
use parking_lot::{Condvar, Mutex};
use storage::DiskManager;

pub type LogBuffer = [u8; LOG_BUFFER_SIZE];
pub type FlushBuffer = [u8; LOG_BUFFER_SIZE];

pub struct LogManager {
    // TODO(students): you may add your own member variables

    /** The atomic counter which records the next log sequence number. */
    next_lsn: AtomicLSN,

    /** The log records before and including the persistent lsn have been written to disk. */
    persistent_lsn: AtomicLSN,

    log_buffer: LogBuffer,
    flush_buffer: FlushBuffer,

    latch: Mutex<()>,

    // TODO - add
    // flush_thread: JoinHandle<()>,
    // std::thread *flush_thread_ __attribute__((__unused__));

    cv: Condvar,

    disk_manager: Box<dyn DiskManager>,
}
