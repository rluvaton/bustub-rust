use crate::buffer::buffer_pool_manager::{PinReadPageGuard, PinWritePageGuard};
use crate::buffer::{AccessType, BufferPoolManager};
use common::config::PageId;
use parking_lot::lock_api::RawRwLock;
use std::ops::Deref;
use std::sync::Arc;
use crate::storage::Page;

#[clippy::has_significant_drop]
#[must_use = "if unused the PinPageGuard will immediately unpin"]
pub struct PinPageGuard {
    pub(crate) page: Page,
    bpm: Arc<BufferPoolManager>,
}

impl PinPageGuard {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        PinPageGuard {
            page,
            bpm: Arc::clone(&bpm),
        }
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
    /// # Example
    ///
    /// ```
    /// use std::sync::Arc;
    /// use parking_lot::Mutex;
    /// use buffer::{BufferPoolManager, PinPageGuard};
    /// use storage::DiskManagerUnlimitedMemory;
    /// let bpm = Arc::new(BufferPoolManager::new(
    ///     10,
    ///     Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new())),
    ///     Some(2),
    ///     None
    /// ));
    ///
    /// let mut page = bpm.new_page().expect("Should be able to create a page");
    /// let guard = PinPageGuard::new(Arc::clone(&bpm), page.clone());
    ///
    /// let read_pin_guard = guard.upgrade_read();
    /// ```
    ///
    /// ```compile_fail
    /// use std::sync::Arc;
    /// use parking_lot::Mutex;
    /// use buffer::{BufferPoolManager, PinPageGuard};
    /// use storage::DiskManagerUnlimitedMemory;
    /// let bpm = Arc::new(BufferPoolManager::new(
    ///     10,
    ///     Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new())),
    ///     Some(2),
    ///     None
    /// ));
    ///
    /// let mut page = bpm.new_page().expect("Should be able to create a page");
    /// let guard = PinPageGuard::new(Arc::clone(&bpm), page.clone());
    ///
    /// let read_pin_guard = guard.upgrade_read();
    ///
    /// // Cannot move out of this
    /// guard.get_page_id();
    /// ```
    ///
    pub fn upgrade_read<'a>(self) -> PinReadPageGuard<'a> {
        let page = self.page.clone();
        PinReadPageGuard::from_guard(self, page)
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
    pub fn upgrade_write<'a>(self) -> PinWritePageGuard<'a> {
        let page = self.page.clone();
        PinWritePageGuard::from_guard(self, page)
    }

    pub fn get_page_id(&self) -> PageId {
        self.page.with_read(|u| u.get_page_id())
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
    pub fn replace_inner(&mut self, page: &mut Page) {
        unimplemented!()
        // // TODO - pin current
        // // TODO - check if should acquire page
        // page.pin();
        //
        // let mut old_page = self.page.clone();
        // self.page = page.clone();
        //
        // // TODO - unpin should not require lock
        // old_page.unpin();
        //
        // unimplemented!()
    }
}

impl Deref for PinPageGuard {
    type Target = Page;

    #[inline]
    fn deref(&self) -> &Page {
        &self.page
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
impl Drop for PinPageGuard {
    fn drop(&mut self) {
        let (page_id, is_dirty) = self.page.with_read(|u| (u.get_page_id(), u.is_dirty()));

        self.bpm.unpin_page(page_id, is_dirty, AccessType::default());
    }
}

#[cfg(test)]
mod tests {
    use crate::buffer::buffer_pool_manager::PinPageGuard;
    use crate::buffer::BufferPoolManager;
    use parking_lot::Mutex;
    use std::sync::Arc;
    use crate::storage::DiskManagerUnlimitedMemory;

    #[test]
    fn creating_pin_page_guard_should_not_increase_pin_count() {
        let bpm = Arc::new(BufferPoolManager::new(
            10,
            Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new())),
            Some(2),
            None,
        ));

        let mut page = bpm.new_page().expect("Should be able to create a page");

        assert_eq!(page.get_pin_count(), 1, "new_page should return pinned page");

        let guard = PinPageGuard::new(Arc::clone(&bpm), page.clone());
        assert_eq!(page.get_pin_count(), 1, "Creating PinPageGuard should not change the pin count");
    }

    #[test]
    fn dropping_pin_page_guard_should_unpin() {
        let bpm = Arc::new(BufferPoolManager::new(
            10,
            Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new())),
            Some(2),
            None,
        ));

        let mut page = bpm.new_page().expect("Should be able to create a page");
        assert_eq!(page.get_pin_count(), 1, "new_page should return pinned page");

        let mut other_ref1 = page.clone();
        let mut other_ref2 = page.clone();

        // Pin twice more to check that we are not unpinning twice when having both guard
        other_ref1.pin();
        other_ref2.pin();

        assert_eq!(page.get_pin_count(), 3, "should have pin count 3");

        {
            let guard = PinPageGuard::new(Arc::clone(&bpm), page.clone());
            assert_eq!(guard.get_pin_count(), 3, "Creating PinPageGuard should not change the pin count");
        }

        assert_eq!(page.get_pin_count(), 2, "Dropping pin page guard should decrease the pin count");
    }


    #[test]
    fn should_not_unpin_when_getting_read_pin_guard_from_guard() {
        let bpm = Arc::new(BufferPoolManager::new(
            10,
            Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new())),
            Some(2),
            None,
        ));

        let mut page = bpm.new_page().expect("Should be able to create a page");
        assert_eq!(page.get_pin_count(), 1, "new_page should return pinned page");

        let mut other_ref1 = page.clone();
        let mut other_ref2 = page.clone();

        // Pin twice more to check that we are not unpinning twice when having both guard
        other_ref1.pin();
        other_ref2.pin();

        assert_eq!(page.get_pin_count(), 3, "should have pin count 3");

        {
            let guard = PinPageGuard::new(Arc::clone(&bpm), page.clone());
            assert_eq!(guard.get_pin_count(), 3, "Creating PinPageGuard should not change the pin count");

            let read_pin_guard = guard.upgrade_read();

            assert_eq!(read_pin_guard.get_pin_count(), 3, "Creating read guard from regular guard should keep pin count");
        }

        assert_eq!(page.get_pin_count(), 2, "Dropping read guard should unpin only once");
    }

    #[test]
    fn should_not_unpin_when_getting_write_pin_guard_from_guard() {
        let bpm = Arc::new(BufferPoolManager::new(
            10,
            Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new())),
            Some(2),
            None,
        ));

        let mut page = bpm.new_page().expect("Should be able to create a page");
        assert_eq!(page.get_pin_count(), 1, "new_page should return pinned page");

        let mut other_ref1 = page.clone();
        let mut other_ref2 = page.clone();

        // Pin twice more to check that we are not unpinning twice when having both guard
        other_ref1.pin();
        other_ref2.pin();

        assert_eq!(page.get_pin_count(), 3, "should have pin count 3");

        {
            let mut guard = PinPageGuard::new(Arc::clone(&bpm), page.clone());
            assert_eq!(guard.get_pin_count(), 3, "Creating PinPageGuard should not change the pin count");

            let write_pin_guard = guard.upgrade_write();

            assert_eq!(write_pin_guard.get_pin_count(), 3, "Creating write guard from regular guard should keep pin count");
        }

        assert_eq!(page.get_pin_count(), 2, "Dropping write guard should unpin only once");
    }
}
