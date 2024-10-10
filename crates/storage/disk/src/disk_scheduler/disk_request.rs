use common::{Promise};
use pages::{PageData, PageId, PageReadGuard, PageWriteGuard, UnderlyingPage};
use crate::page_data_transfer::MutPageDataTransfer;
use crate::PageDataTransfer;

/**
 * @brief Represents a Read request for the DiskManager to execute.
 */
pub struct ReadDiskRequest {
    /**
     *  Pointer to the start of the memory location where a page is being read into from disk (on a read).
     */
    pub data: MutPageDataTransfer,

    /** ID of the page being read from disk. */
    pub page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

impl ReadDiskRequest {
    pub fn new(page_id_to_fetch: PageId, dest: PageWriteGuard, callback: Promise<bool>) -> Self {
        ReadDiskRequest {
            page_id: page_id_to_fetch,
            data: dest.into(),
            callback,
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
    pub(super) data: PageDataTransfer,

    /** ID of the page being written to disk. */
    pub(super) page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub(super) callback: Promise<bool>,
}


impl WriteDiskRequest {
    pub fn new(page_id_to_write_to: PageId, src: PageReadGuard, callback: Promise<bool>) -> Self {
        WriteDiskRequest {
            page_id: page_id_to_write_to,
            data: src.into(),
            callback,
        }
    }
}

/**
 * @brief Represents a Write request for the DiskManager to execute.
 */
pub struct ReadWriteDiskRequest<'a> {
    /**
     *  Pointer to the start of the memory location where a page being written out to disk

    Having box will reduce performance as it will need to create in the heap
     */
    pub(super) data: &'a mut PageData,

    /** ID of the page being written to disk. */
    pub(super) page_id_to_write: PageId,

    // which page to read after write
    pub(super) page_id_to_read: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub(super) callback: Promise<bool>,
}


impl<'a> ReadWriteDiskRequest<'a> {
    pub fn new(page_id_to_write_to: PageId, page_id_to_read: PageId, mut src_and_dest: PageWriteGuard<'a>, callback: Promise<bool>) -> Self {
        ReadWriteDiskRequest {
            page_id_to_write: page_id_to_write_to,
            page_id_to_read,

            data: src_and_dest.get_data_mut(),
            callback,
        }
    }
}

pub enum DiskRequestType {
    Read(ReadDiskRequest),
    Write(WriteDiskRequest),
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
