use std::mem;
use super::super::BufferPoolManager;
use crate::storage::PageAndReadGuard;
use std::sync::Arc;
use common::config::{PageData, PageId};
use crate::buffer::AccessType;
use crate::buffer::buffer_pool_manager_1::BufferPool;

/// Read guard on page that will also unpin on drop
#[clippy::has_significant_drop]
#[must_use = "if unused the PageReadGuard will immediately unpin and release write guard"]
pub struct PageReadGuard<'a> {
    bpm: Arc<BufferPoolManager>,

    page_and_read_guard: Option<PageAndReadGuard<'a>>
}

impl<'a> PageReadGuard<'a> {
    pub(in super::super) fn new(bpm: Arc<BufferPoolManager>, page: PageAndReadGuard<'a>) -> Self {
        Self {
            bpm,

            // The option is done only for the custom drop
            page_and_read_guard: Some(page),
        }
    }

    pub fn get_page_id(&self) -> PageId {
        match &self.page_and_read_guard {
            Some(p) => p.get_page_id(),
            None => unreachable!()
        }
    }

    pub fn cast<T>(&self) -> &T {
        match &self.page_and_read_guard {
            Some(p) => p.cast::<T>(),
            None => unreachable!()
        }
    }

    /// @return the actual data contained within this page
    pub fn get_data(&self) -> &PageData {
        match &self.page_and_read_guard {
            Some(p) => p.get_data(),
            None => unreachable!()
        }
    }
}

impl<'a> Drop for PageReadGuard<'a> {
    fn drop(&mut self) {
        // Must always have page and read guard,
        let page_and_read_guard = mem::take(&mut self.page_and_read_guard).unwrap();
        let page_id = page_and_read_guard.get_page_id().clone();
        let bpm = self.bpm.clone();

        // Drop the write guard
        drop(page_and_read_guard);

        // Unpin page
        bpm.unpin_page(page_id, AccessType::Unknown);
    }
}
