use std::mem::size_of;
use common::config::{PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID, LSN};
use common::ReaderWriterLatch;

type PageData = [u8; BUSTUB_PAGE_SIZE];

// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);

/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug)]
pub struct Page {
    /** The actual data that is stored within a page. */
    // Usually this should be stored as `char data_[BUSTUB_PAGE_SIZE]{};`. But to enable ASAN to detect page overflow,
    // we store it as a ptr.
    data: PageData,

    /** The ID of this page. */
    // TODO - default = INVALID_PAGE_ID;
    page_id: PageId,

    /** The pin count of this page. */
    pin_count: i32,
    /** True if the page is dirty, i.e. it is different from its corresponding page on disk. */
    is_dirty: bool,

    /** Page latch. */
    rwlatch: ReaderWriterLatch<()>,
}


impl Page {
    const SIZE_PAGE_HEADER: usize = 8;
    const OFFSET_PAGE_START: usize = 0;
    const OFFSET_LSN: usize = 4;

    pub fn new() -> Self {
        Page {
            page_id: INVALID_PAGE_ID,
            is_dirty: false,
            pin_count: 0,
            rwlatch: ReaderWriterLatch::new(()),
            data: [0u8; BUSTUB_PAGE_SIZE],
        }
    }

    /** @return the actual data contained within this page */
    pub fn get_data(&self) -> &PageData {
        &self.data
    }

    /** @return the actual data contained within this page */
    pub fn get_data_mut(&mut self) -> &mut PageData {
        &mut self.data
    }

    /** @return the page id of this page */
    pub fn get_page_id(&self) -> PageId {
        self.page_id
    }

    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> i32 {
        self.pin_count
    }

    /** @return true if the page in memory has been modified from the page on disk, false otherwise */
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    pub fn get_read_write_latch(&self) -> &ReaderWriterLatch<()> {
        &self.rwlatch
    }


    /** @return the page LSN. */
    pub fn get_lsn(&self) -> LSN {
        // return *reinterpret_cast<lsn_t *>(GetData() + OFFSET_LSN);
        LSN::from_ne_bytes(self.data[Page::OFFSET_LSN..Page::OFFSET_LSN + size_of::<LSN>()].try_into().unwrap())
    }

    /** Sets the page LSN. */
    pub fn set_lsn(&mut self, lsn: LSN) {
        self.data[Self::OFFSET_LSN..].copy_from_slice(lsn.to_ne_bytes().as_slice());
        // memcpy(GetData() + OFFSET_LSN, &lsn, sizeof(lsn_t));
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.page_id.eq(&other.page_id) &&
            self.pin_count.eq(&other.pin_count) &&
            self.is_dirty.eq(&other.is_dirty) &&
            self.data.eq(&other.data)
    }
}
