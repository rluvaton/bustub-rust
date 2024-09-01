use crate::buffer_pool_manager::BasicPageGuard;
use crate::BufferPoolManager;
use common::config::{PageData, PageId};
use std::sync::Arc;
use storage::Page;

pub struct ReadPageGuard {
    guard: BasicPageGuard,
}

impl ReadPageGuard {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        ReadPageGuard {
            guard: BasicPageGuard::new(bpm, page)
        }
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Move constructor for ReadPageGuard
    ///
    /// Very similar to BasicPageGuard. You want to create
    /// a ReadPageGuard using another ReadPageGuard. Think
    /// about if there's any way you can make this easier for yourself...
    ///
    pub fn from(other: Self) {
        unimplemented!();
    }

    pub fn get_page_id(&self) -> PageId {
        self.guard.get_page_id()
    }

    pub fn get_data(&self) -> &PageData {
        self.guard.get_data()
    }

    pub fn cast<T>(&self) -> &T {
        self.guard.cast()
    }

    /// * TODO(P2): Add implementation
    ///
    ///  @brief Move assignment for ReadPageGuard
    ///
    ///  Very similar to BasicPageGuard. Given another ReadPageGuard,
    ///  replace the contents of this one with that one.
    ///
    pub fn replace_inner(&mut self) {}
}

///  TODO(P2): Add implementation
///
/// @brief Drop a ReadPageGuard
///
/// ReadPageGuard's Drop should behave similarly to BasicPageGuard,
/// except that ReadPageGuard has an additional resource - the latch!
/// However, you should think VERY carefully about in which order you
/// want to release these resources.
///
impl Drop for ReadPageGuard {
    fn drop(&mut self) {
        todo!()
    }
}
