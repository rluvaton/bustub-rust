mod manager_impl;

use common::config::{AtomicLSN};
use parking_lot::{Condvar, Mutex};
use disk_storage::DiskManager;
use crate::types::{FlushBuffer, LogBuffer};

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
