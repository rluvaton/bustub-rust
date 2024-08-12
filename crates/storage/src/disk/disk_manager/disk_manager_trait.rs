use std::ffi::c_void;
use std::fs::File;
use std::future::Future;
use std::sync::Mutex;
use common::config::PageId;

/**
 * DiskManager takes care of the allocation and deallocation of pages within a database. It performs the reading and
 * writing of pages to and from disk, providing a logical file layer within the context of a database management system.
 */
trait DiskManagerT {
    /**
     * Creates a new disk manager that writes to the specified database file.
     * @param db_file the file name of the database file to write to
     */
    fn new(db_file: String) -> Self;

    /**
     * Shut down the disk manager and close all the file resources.
     */
    fn shut_down() {
        unimplemented!()
    }


    /**
     * Write a page to the database file.
     * @param page_id id of the page
     * @param page_data raw page data
     */
    fn write_page(&self, page_id: PageId, page_data: &[u8]) {
        unimplemented!()
    }

    /**
     * Read a page from the database file.
     * @param page_id id of the page
     * @param[out] page_data output buffer
     */
    fn read_page(&self, page_id: PageId, page_data: &[u8]) {
        unimplemented!()
    }

    /**
     * Flush the entire log buffer into disk.
     * @param log_data raw log data
     * @param size size of log entry
     */
    fn write_log(log_data: &[u8], size: i32) {
        unimplemented!()
    }

    /**
     * Read a log entry from the log file.
     * @param[out] log_data output buffer
     * @param size size of the log entry
     * @param offset offset of the log entry in the file
     * @return true if the read was successful, false otherwise
     */
    fn read_log(log_data: &[u8], size: i32, offset: i32) -> bool {
        unimplemented!()
    }

    /** @return the number of disk flushes */
    fn get_num_flushes() -> i32 {
        unimplemented!()
    }

    /** @return true iff the in-memory content has not been flushed yet */
    fn get_flush_state() -> bool {
        unimplemented!()
    }

    /** @return the number of disk writes */
    fn get_num_writes() -> i32 {
        unimplemented!()
    }

    /**
     * Sets the future which is used to check for non-blocking flushes.
     * @param f the non-blocking flush check
     */
    fn set_flush_log_future(f: Box<dyn Future<Output=()>>) {
        // flush_log_f_ = f;
        unimplemented!()
    }

    /** Checks if the non-blocking flush future was set. */
    fn has_flush_log_future() -> bool {
        // return flush_log_f_ != nullptr;
        unimplemented!()
    }


    fn get_file_size(file_name: String) -> i32 {
        unimplemented!()
    }
}



