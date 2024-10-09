use std::fmt::{Debug, Formatter};
use super::super::BufferPoolManager;
use buffer_common::AccessType;
use pages::{PageAndGuard, PageAndWriteGuard,PageData, PageId};
use std::mem;
use std::sync::Arc;

/// Read guard on page that will also unpin on drop
#[clippy::has_significant_drop]
#[must_use = "if unused the PageWriteGuard will immediately unpin and release write guard"]
pub struct PageWriteGuard<'a> {
    bpm: Arc<BufferPoolManager>,

    page_and_write_guard: Option<PageAndWriteGuard<'a>>
}

impl<'a> PageWriteGuard<'a> {
    pub(in super::super) fn new(bpm: Arc<BufferPoolManager>, page: PageAndWriteGuard<'a>) -> Self {
        Self {
            bpm,

            // The option is done only for the custom drop
            page_and_write_guard: Some(page),
        }
    }

    pub fn get_page_id(&self) -> PageId {
        match &self.page_and_write_guard {
            Some(p) => p.get_page_id(),
            None => unreachable!()
        }
    }

    pub fn cast<T>(&self) -> &T {
        match &self.page_and_write_guard {
            Some(p) => p.cast::<T>(),
            None => unreachable!()
        }
    }

    pub fn cast_mut<T>(&mut self) -> &mut T {
        match &mut self.page_and_write_guard {
            Some(p) => {
                p.page().set_is_dirty(true);

                p.cast_mut::<T>()
            },
            None => unreachable!()
        }
    }

    /// @return the actual data contained within this page
    pub fn get_data(&self) -> &PageData {
        match &self.page_and_write_guard {
            Some(p) => p.get_data(),
            None => unreachable!()
        }
    }

    /// Get Mutable reference to the data and set the dirty flag to true
    ///
    /// Returns: the actual data contained within this page
    pub fn get_data_mut(&mut self) -> &mut PageData {

        match &mut self.page_and_write_guard {
            Some(p) => {
                p.page().set_is_dirty(true);

                p.get_data_mut()
            },
            None => unreachable!()
        }
    }
}

impl<'a> Drop for PageWriteGuard<'a> {
    fn drop(&mut self) {
        // Must always have page and write guard,
        let page_and_write_guard = mem::take(&mut self.page_and_write_guard).unwrap();
        let page_id = page_and_write_guard.get_page_id().clone();
        let bpm = self.bpm.clone();

        // Drop the write guard
        drop(page_and_write_guard);

        // Unpin page
        bpm.unpin_page(page_id, AccessType::Unknown);
    }
}

impl Debug for PageWriteGuard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "page write guard for page id {}", self.get_page_id())
    }
}
