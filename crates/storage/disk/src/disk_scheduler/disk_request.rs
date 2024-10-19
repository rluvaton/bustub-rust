use common::Promise;
use pages::{PageData, PageId};

/**
 * @brief Represents a Read request for the DiskManager to execute.
 */
pub struct ReadDiskRequest<'a> {
    /**
     *  Pointer to the start of the memory location where a page is being read into from disk (on a read).
     */
    pub data: &'a mut PageData,

    /** ID of the page being read from disk. */
    pub page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

impl<'a> ReadDiskRequest<'a> {
    pub fn new(source_page_id: PageId, dest_data: &'a mut PageData, callback: Promise<bool>) -> Self {
        ReadDiskRequest {
            page_id: source_page_id,
            data: dest_data,
            callback,
        }
    }
}

/**
 * @brief Represents a Write request for the DiskManager to execute.
 */
pub struct WriteDiskRequest<'a> {
    /**
     *  Pointer to the start of the memory location where a page being written out to disk

    Having box will reduce performance as it will need to create in the heap
     */
    pub(super) data: &'a PageData,

    /** ID of the page being written to disk. */
    pub(super) page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub(super) callback: Promise<bool>,
}


impl<'a> WriteDiskRequest<'a> {
    pub fn new(dest_page_id: PageId, source_data: &'a PageData, callback: Promise<bool>) -> Self {
        WriteDiskRequest {
            page_id: dest_page_id,
            data: source_data,
            callback,
        }
    }
}

/// Single message for both read and write to the same page data buffer
///
/// First writing the data provided to the dest_page_id and then reading from source_page_id into the data (replacing the content)
pub struct WriteAndReadDiskRequest<'a> {
    /// Data to write to disk and to read from disk
    pub data: &'a mut PageData,

    /// The ID of the page that first going to write the data
    pub dest_page_id: PageId,

    // The ID of the page that after write is going to be read from disk
    pub source_page_id: PageId,

    /** Callback used to signal to the request issuer when the request has been completed. */
    pub callback: Promise<bool>,
}

impl<'a> WriteAndReadDiskRequest<'a> {
    pub fn new(dest_page_id: PageId, source_page_id: PageId, data: &'a mut PageData, callback: Promise<bool>) -> Self {
        WriteAndReadDiskRequest {
            dest_page_id,
            source_page_id,
            data,
            callback,
        }
    }
}

pub enum DiskRequestType<'a> {
    Read(ReadDiskRequest<'a>),
    Write(WriteDiskRequest<'a>),
    WriteAndRead(WriteAndReadDiskRequest<'a>),
}

impl<'a> From<ReadDiskRequest<'a>> for DiskRequestType<'a> {
    fn from(value: ReadDiskRequest<'a>) -> Self {
        DiskRequestType::Read(value)
    }
}

impl<'a> From<WriteDiskRequest<'a>> for DiskRequestType<'a> {
    fn from(value: WriteDiskRequest<'a>) -> Self {
        DiskRequestType::Write(value)
    }
}

impl<'a> From<WriteAndReadDiskRequest<'a>> for DiskRequestType<'a> {
    fn from(value: WriteAndReadDiskRequest<'a>) -> Self {
        DiskRequestType::WriteAndRead(value)
    }
}
