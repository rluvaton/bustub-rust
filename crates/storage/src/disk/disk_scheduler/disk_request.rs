use std::thread::JoinHandle;
use common::config::PageId;

/**
 * @brief Represents a Write or Read request for the DiskManager to execute.
 */
pub(crate) struct DiskRequest<'a> {

    /** Flag indicating whether the request is a write or a read. */
    pub(crate) is_write: bool,

    /**
     *  Pointer to the start of the memory location where a page is either:
     *   1. being read into from disk (on a read).
     *   2. being written out to disk (on a write).
     */
    // char *data_;
    //
    data: &'a [u8],

    /** ID of the page being read from / written to disk. */
    page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    // std::promise<bool> callback_;
    callback: JoinHandle<bool>,
}
