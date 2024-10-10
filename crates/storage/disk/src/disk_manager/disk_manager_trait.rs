use pages::PageId;
use common::{Future, SharedFuture};

/**
 * DiskManager takes care of the allocation and deallocation of pages within a database. It performs the reading and
 * writing of pages to and from disk, providing a logical file layer within the context of a database management system.
 */
pub trait DiskManager: Sync + Send {

    /**
     * Shut down the disk manager and close all the file resources.

    // TODO - don't really need, can implement the drop?
     */
    fn shut_down(&mut self);


    /**
     * Write a page to the database file.
     * @param page_id id of the page
     * @param page_data raw page data
     */
    fn write_page(&mut self, page_id: PageId, page_data: &[u8]);

    /**
     * Read a page from the database file.
     * @param page_id id of the page
     * @param[out] page_data output buffer
     */
    fn read_page(&mut self, page_id: PageId, page_data: &mut [u8]);

    /**
     * Flush the entire log buffer into disk.
     * @param log_data raw log data
     * @param size size of log entry
     */
    fn write_log(&mut self, log_data: &[u8], size: i32);

    /**
     * Read a log entry from the log file.
     * @param[out] log_data output buffer
     * @param size size of the log entry
     * @param offset offset of the log entry in the file
     * @return true if the read was successful, false otherwise
     */
    fn read_log(&mut self, log_data: &mut [u8], size: i32, offset: i32) -> bool;

    /** @return the number of disk flushes */
    fn get_num_flushes(&self) -> i32;

    /** @return true iff the in-memory content has not been flushed yet */
    fn get_flush_state(&self) -> bool;

    /** @return the number of disk writes */
    fn get_num_writes(&self) -> i32;

    /**
     * Sets the future which is used to check for non-blocking flushes.
     * @param f the non-blocking flush check
     */
    fn set_flush_log_future(&mut self, f: Option<SharedFuture<()>>);

    /** Checks if the non-blocking flush future was set. */
    fn has_flush_log_future(&self) -> bool;
}



