use common::config::{LSN};
use std::mem::size_of;
use crate::{PageData, PageId, INVALID_PAGE_ID, PAGE_SIZE};

//noinspection RsAssertEqual
const _:() = assert!(size_of::<PageId>() == 4);
//noinspection RsAssertEqual
const _:() = assert!(size_of::<LSN>() == 4);


/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug, PartialEq)]
pub struct UnderlyingPage {
    /** The actual data that is stored within a page. */
    // Usually this should be stored as `char data_[BUSTUB_PAGE_SIZE]{};`. But to enable ASAN to detect page overflow,
    // we store it as a ptr.
    data: PageData,

    /** The ID of this page. */
    // TODO - default = INVALID_PAGE_ID;
    page_id: PageId,
}

impl UnderlyingPage {
    #[allow(unused)]
    const SIZE_PAGE_HEADER: usize = 8;
    #[allow(unused)]
    const OFFSET_PAGE_START: usize = 0;
    #[allow(unused)]
    const OFFSET_LSN: usize = 4;

    pub(crate) fn new(page_id: PageId, data: PageData) -> Self {
        UnderlyingPage {
            page_id,
            // is_dirty: false,
            // pin_count: 0,
            data,
        }
    }

    /** @return the actual data contained within this page */
    pub fn get_data(&self) -> &PageData {
        &self.data
    }

    /// Get Mutable reference to the data and set the dirty flag to true
    ///
    /// Returns: the actual data contained within this page
    pub fn get_data_mut(&mut self) -> &mut PageData {
        // Set as dirty if getting mutable data
        // self.is_dirty = true;

        &mut self.data
    }

    // TODO - make sure that once the guard is dropped, this is unusable
    pub fn cast<T>(&self) -> &T {
        unsafe { &*(self.data.as_ptr() as *const PageData as *const T) }
    }

    pub fn cast_mut<T>(&mut self) -> &mut T {
        // Set as dirty if getting mutable data
        // self.is_dirty = true;

        unsafe { &mut *(self.data.as_mut_ptr() as *mut PageData as *mut T) }
    }

    /// Get Mutable reference to the data
    ///
    /// Returns: the actual data contained within this page
    ///
    /// # Safety
    /// This is unsafe as it will not set `is_dirty` to true, prefer using `UnderlyingPage::get_data_mut`
    pub unsafe fn get_data_mut_unchecked(&mut self) -> &mut PageData {
        &mut self.data
    }

    /** @return the page id of this page */
    pub fn get_page_id(&self) -> PageId {
        self.page_id
    }

    /** @return the page LSN. */
    pub fn get_lsn(&self) -> LSN {
        // return *reinterpret_cast<lsn_t *>(GetData() + OFFSET_LSN);
        LSN::from_ne_bytes(self.data[UnderlyingPage::OFFSET_LSN..UnderlyingPage::OFFSET_LSN + size_of::<LSN>()].try_into().unwrap())
    }

    /** Sets the page LSN. */
    pub fn set_lsn(&mut self, lsn: LSN) {
        self.data[Self::OFFSET_LSN..].copy_from_slice(lsn.to_ne_bytes().as_slice());
        // memcpy(GetData() + OFFSET_LSN, &lsn, sizeof(lsn_t));
    }

    /// Clear page id and data so it will be like a new page
    pub fn clear_page(&mut self, page_id: PageId) {
        self.page_id = page_id;
        self.data.fill(0);
    }

    /// Change th page id, this should only be used in buffer pools!
    pub fn set_page_id(&mut self, page_id: PageId) {
        self.page_id = page_id;
    }

    pub fn set_data(&mut self, data: PageData) {
        self.data = data;
    }
}

impl Default for UnderlyingPage {
    fn default() -> Self {
        UnderlyingPage::new(INVALID_PAGE_ID, [0u8; PAGE_SIZE])
    }
}

