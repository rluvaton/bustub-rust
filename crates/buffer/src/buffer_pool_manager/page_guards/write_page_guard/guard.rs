use crate::buffer_pool_manager::BasicPageGuard;
use crate::BufferPoolManager;
use common::config::{PageData, PageId};
use std::sync::Arc;
use storage::Page;

pub struct WritePageGuard {
    guard: BasicPageGuard,
}


impl WritePageGuard {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        WritePageGuard {
            guard: BasicPageGuard::new(bpm, page)
        }
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Move constructor for WritePageGuard
    ///
    /// Very similar to BasicPageGuard. You want to create
    /// a WritePageGuard using another WritePageGuard. Think
    /// about if there's any way you can make this easier for yourself...
    ///
    pub fn from(other: Self) {
        unimplemented!();
    }

    pub fn get_page_id(&self) -> PageId {
        self.guard.get_page_id()
    }

    pub fn get_data_mut(&self) -> &mut PageData {
        self.guard.get_data_mut()
    }

    pub fn cast<T>(&self) -> &T {
        self.guard.cast()
    }

    pub fn cast_mut<T>(&self) -> &mut T {
        self.guard.cast_mut()
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Move assignment for WritePageGuard
    ///
    /// Very similar to BasicPageGuard. Given another WritePageGuard,
    /// replace the contents of this one with that one.
    ///
    pub fn replace_inner(&mut self) {
        unimplemented!()
    }
}

///  TODO(P2): Add implementation
///
/// @brief Drop a WritePageGuard
///
/// WritePageGuard's Drop should behave similarly to BasicPageGuard,
/// except that WritePageGuard has an additional resource - the latch!
/// However, you should think VERY carefully about in which order you
/// want to release these resources.
///
impl Drop for WritePageGuard {
    fn drop(&mut self) {
        todo!()
    }
}
