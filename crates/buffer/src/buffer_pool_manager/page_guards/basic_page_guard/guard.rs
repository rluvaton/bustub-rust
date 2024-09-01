use std::sync::Arc;
use common::config::{PageData, PageId};
use storage::Page;
use crate::buffer_pool_manager::{ReadPageGuard, WritePageGuard};
use crate::BufferPoolManager;

pub struct BasicPageGuard {
    page: Page,
    bpm: Arc<BufferPoolManager>
}

impl BasicPageGuard {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        BasicPageGuard {
            page,
            bpm: Arc::clone(&bpm),
        }
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Move constructor for BasicPageGuard
    ///
    /// When you call BasicPageGuard(std::move(other_guard)), you
    /// expect that the new guard will behave exactly like the other
    /// one. In addition, the old page guard should not be usable. For
    /// example, it should not be possible to call .Drop() on both page
    /// guards and have the pin count decrease by 2.
    ///
    pub fn from(other: Self) {
        unimplemented!();
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Upgrade a BasicPageGuard to a ReadPageGuard
    ///
    /// The protected page is not evicted from the buffer pool during the upgrade,
    /// and the basic page guard should be made invalid after calling this function.
    ///
    /// @return an upgraded ReadPageGuard
    ///
    pub fn upgrade_read(&mut self) -> ReadPageGuard {
        unimplemented!()
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Upgrade a BasicPageGuard to a WritePageGuard
    ///
    /// The protected page is not evicted from the buffer pool during the upgrade,
    /// and the basic page guard should be made invalid after calling this function.
    ///
    /// @return an upgraded WritePageGuard
    ///
    pub fn upgrade_write(&mut self) -> WritePageGuard {
        unimplemented!()
    }

    pub fn get_page_id(&self) -> PageId {
        self.page.with_read(|u| u.get_page_id())
    }

    pub fn get_data(&self) -> &PageData {
        // TODO - hold read lock
        // self.page.with_read(|u| u.get_data())
        unimplemented!()

    }

    pub fn get_data_mut(&self) -> &mut PageData {
        // todo - set is dirty here?
        // TODO - hold write lock
        // self.page.with_write(|mut u| u.get_data_mut())
        unimplemented!()
    }

    pub fn cast<T>(&self) -> &T {
        unimplemented!()
    }

    pub fn cast_mut<T>(&self) -> &mut T {
        unimplemented!()
    }

    /// TODO(P2): Add implementation
    ///
    /// @brief Move assignment for BasicPageGuard
    ///
    /// Similar to a move constructor, except that the move
    /// assignment assumes that BasicPageGuard already has a page
    /// being guarded. Think carefully about what should happen when
    /// a guard replaces its held page with a different one, given
    /// the purpose of a page guard.
    ///
    pub fn replace_inner(&mut self) {

    }
}

/// TODO(P2): Add implementation
///
/// @brief Drop a page guard
///
/// Dropping a page guard should clear all contents
/// (so that the page guard is no longer useful), and
/// it should tell the BPM that we are done using this page,
/// per the specification in the writeup.
///
impl Drop for BasicPageGuard {
    fn drop(&mut self) {
        todo!()
    }
}
