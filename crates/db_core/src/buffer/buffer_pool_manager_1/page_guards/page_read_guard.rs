use std::sync::Arc;
use crate::buffer::buffer_pool_manager_1::PageWriteGuard;
use crate::storage::Page;
use super::super::BufferPoolManager;


/// Read guard on page that will also unpin on drop
#[clippy::has_significant_drop]
#[must_use = "if unused the PageReadGuard will immediately unpin and release read guard"]
pub struct PageReadGuard<'a> {
    bpm: Arc<BufferPoolManager>,

    // TODO - this is not write
    page: Page,
}


impl<'a> PageReadGuard<'a> {
    pub(in super::super) fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        todo!();
    }
}


impl<'a> Drop for PageReadGuard<'a> {
    fn drop(&mut self) {
        // 1. Unpin page
        // let page_id = self.page.;
        // 2. release lock
        todo!()
    }
}
