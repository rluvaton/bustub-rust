use std::sync::Arc;
use common::config::PageId;
use crate::buffer::AccessType;
use crate::buffer::buffer_pool_manager_1::BufferPool;
use super::super::BufferPoolManager;
use crate::storage::Page;

/// Read guard on page that will also unpin on drop
#[clippy::has_significant_drop]
#[must_use = "if unused the PageWriteGuard will immediately unpin and release write guard"]
pub struct PageWriteGuard<'a> {
    bpm: Arc<BufferPoolManager>,

    page_id: PageId,

    // TODO - this is not write
    page: Page,
}

impl<'a> PageWriteGuard<'a> {
    pub(in super::super) fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        todo!();
    }

    pub fn get_page_id(&self) -> PageId {
        self.page_id
    }

    pub fn cast<T>(&self) -> &T {
        todo!()
    }

    pub fn cast_mut<T>(&mut self) -> &mut T {
        todo!()
    }
}

impl<'a> Drop for PageWriteGuard<'a> {
    fn drop(&mut self) {
        let page_id = self.page_id;

        // 1. drop write guard

        // 2. Unpin page?
        self.bpm.unpin_page(self.page_id, AccessType::Unknown);

        // let page_id = self.page.;
        // 2. release lock
        todo!()
    }
}
