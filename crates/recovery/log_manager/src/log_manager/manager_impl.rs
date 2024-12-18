use std::sync::Arc;
use std::sync::atomic::Ordering;
use parking_lot::Mutex;
use common::config::{AtomicLSN, INVALID_LSN, LSN};
use disk_storage::DiskManager;
use crate::{LogManager, LogRecord, LOG_BUFFER_SIZE};
use crate::types::LogBuffer;

impl LogManager {

    pub fn new<D: DiskManager>(_disk_manager: Arc<D>) -> Self {
        LogManager {
            next_lsn: AtomicLSN::new(0),
            persistent_lsn: AtomicLSN::new(INVALID_LSN),
            log_buffer: [0u8; LOG_BUFFER_SIZE],
            flush_buffer: [0u8; LOG_BUFFER_SIZE],
            latch: Mutex::new(()),
            // flush_thread: (),
            cv: Default::default(),
            // disk_manager,
        }
    }

    pub fn run_flush_thread(&self) {
        unimplemented!()
    }

    pub fn stop_flush_thread(&self) {
        // unimplemented!()
    }

    pub fn append_log_record(&mut self, _log_record: &LogRecord) -> LSN {
        unimplemented!()
    }

    pub fn get_next_lsn(&self) -> LSN {
        self.next_lsn.load(Ordering::SeqCst)
    }

    pub fn get_persistent_lsn(&self) -> LSN {
        self.persistent_lsn.load(Ordering::SeqCst)
    }

    pub fn set_persistent_lsn(&mut self, lsn: LSN) {
        self.persistent_lsn.store(lsn, Ordering::SeqCst)
    }

    pub fn get_log_buffer(&self) -> LogBuffer {
        self.log_buffer
    }
}
