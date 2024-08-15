use std::sync::{Arc, Mutex};
use common::config::PageId;
use common::Promise;

/**
 * @brief Represents a Write or Read request for the DiskManager to execute.
 */
pub(crate) struct DiskRequest {

    /** Flag indicating whether the request is a write or a read. */
    pub(crate) is_write: bool,

    /**
     *  Pointer to the start of the memory location where a page is either:
     *   1. being read into from disk (on a read).
     *   2. being written out to disk (on a write).
     */
    // char *data_;
    //
    pub(crate) data: Vec<u8>,

    /** ID of the page being read from / written to disk. */
    pub(crate) page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    // std::promise<bool> callback_;
    pub(crate) callback: Promise<bool>,
}


/**
 * @brief Represents a Write or Read request for the DiskManager to execute.
 */
pub(crate) struct ReadDiskRequest<const Size: usize> {
    /**
     *  Pointer to the start of the memory location where a page is being read into from disk (on a read).
     */
    pub(crate) data: Arc<Mutex<[u8; Size]>>,

    /** ID of the page being read from disk. */
    pub(crate) page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub(crate) callback: Promise<bool>,
}


/**
 * @brief Represents a Write or Read request for the DiskManager to execute.
 */
pub(crate) struct WriteDiskRequest<const Size: usize> {
    /**
     *  Pointer to the start of the memory location where a page being written out to disk
     */
    pub(crate) data: Arc<[u8; Size]>,

    /** ID of the page being written to disk. */
    pub(crate) page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub(crate) callback: Promise<bool>,
}

pub(crate) enum DiskRequestType<const Size: usize> {
    Read(ReadDiskRequest<Size>),
    Write(WriteDiskRequest<Size>)
}

impl<const Size: usize> From<ReadDiskRequest<Size>> for DiskRequestType<Size> {
    fn from(value: ReadDiskRequest<Size>) -> Self {
        DiskRequestType::Read(value)
    }
}

impl<const Size: usize> From<WriteDiskRequest<Size>> for DiskRequestType<Size> {
    fn from(value: WriteDiskRequest<Size>) -> Self {
        DiskRequestType::Write(value)
    }
}
