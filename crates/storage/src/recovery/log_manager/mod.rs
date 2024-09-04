mod manager_impl;

use common::config::{AtomicLSN, LOG_BUFFER_SIZE};
use parking_lot::{Condvar, Mutex};
use crate::storage::DiskManager;

pub type LogBuffer = [u8; LOG_BUFFER_SIZE];
pub type FlushBuffer = [u8; LOG_BUFFER_SIZE];

pub struct LogManager {

    /** The atomic counter which records the next log sequence number. */
    #[allow(unused)]
    next_lsn: AtomicLSN,

    /** The log records before and including the persistent lsn have been written to disk. */
    #[allow(unused)]
    persistent_lsn: AtomicLSN,

    #[allow(unused)]
    log_buffer: LogBuffer,
    #[allow(unused)]
    flush_buffer: FlushBuffer,

    #[allow(unused)]
    latch: Mutex<()>,

    // TODO - add
    // flush_thread: JoinHandle<()>,
    // std::thread *flush_thread_ __attribute__((__unused__));

    #[allow(unused)]
    cv: Condvar,

    #[allow(unused)]
    disk_manager: Box<dyn DiskManager>,
}
