use std::sync::{Arc};
use parking_lot::Mutex;
use common::config::{PageData, PageId};
use common::Promise;

/**
 * @brief Represents a Read request for the DiskManager to execute.
 */
pub struct ReadDiskRequest {
    /**
     *  Pointer to the start of the memory location where a page is being read into from disk (on a read).
    Having box will reduce performance as it will need to create in the heap
     */
    pub data: Arc<Mutex<PageData>>,

    /** ID of the page being read from disk. */
    pub page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

impl ReadDiskRequest {
    pub fn new(page_id: PageId, data: Arc<Mutex<PageData>>, callback: Promise<bool>) -> Self {
        ReadDiskRequest {
            page_id,
            data: data.clone(),
            callback
        }
    }
}

/**
 * @brief Represents a Write request for the DiskManager to execute.
 */
pub struct WriteDiskRequest {
    /**
     *  Pointer to the start of the memory location where a page being written out to disk

    Having box will reduce performance as it will need to create in the heap
     */
    pub data: Arc<PageData>,

    /** ID of the page being written to disk. */
    pub page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

impl WriteDiskRequest {
    pub fn new(page_id: PageId, data: Arc<PageData>, callback: Promise<bool>) -> Self {
        WriteDiskRequest {
            page_id,
            data: data.clone(),
            callback
        }
    }
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
