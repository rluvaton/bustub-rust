use std::sync::{Arc};
use parking_lot::Mutex;
use common::config::PageId;
use common::Promise;

/**
 * @brief Represents a Read request for the DiskManager to execute.
 */
pub struct ReadDiskRequest {
    /**
     *  Pointer to the start of the memory location where a page is being read into from disk (on a read).
    Having box will reduce performance as it will need to create in the heap
     */
    pub data: Arc<Mutex<Box<[u8]>>>,

    /** ID of the page being read from disk. */
    pub page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

/**
 * @brief Represents a Write request for the DiskManager to execute.
 */
pub struct WriteDiskRequest {
    /**
     *  Pointer to the start of the memory location where a page being written out to disk

    Having box will reduce performance as it will need to create in the heap
     */
    pub data: Arc<Box<[u8]>>,

    /** ID of the page being written to disk. */
    pub page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

pub enum DiskRequestType {
    Read(ReadDiskRequest),
    Write(WriteDiskRequest)
}

impl From<ReadDiskRequest> for DiskRequestType {
    fn from(value: ReadDiskRequest) -> Self {
        DiskRequestType::Read(value)
    }
}

impl From<WriteDiskRequest> for DiskRequestType {
    fn from(value: WriteDiskRequest) -> Self {
        DiskRequestType::Write(value)
    }
}
