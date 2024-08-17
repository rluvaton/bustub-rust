use std::ops::{Deref, DerefMut};
use crate::page::underlying_page::UnderlyingPage;
use common::config::{PageData, PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use common::ReaderWriterLatch;
use std::sync::{Arc, Weak};
use anyhow::anyhow;
// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);

/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug)]
pub struct Page {
    inner: Arc<ReaderWriterLatch<(usize, Option<UnderlyingPage>)>>,
    is_pinned: bool,
}

impl Page {
    /// Create page with strong reference
    ///
    /// # Arguments
    ///
    /// * `page_id`: the ID of the page
    ///
    /// returns: Page
    ///
    pub fn new(page_id: PageId) -> Self {
        Self::create_with_data(page_id, [0u8; BUSTUB_PAGE_SIZE])
    }

    pub fn create_with_data(page_id: PageId, data: PageData) -> Self {
        Page {
            // Starting as pinned
            is_pinned: true,
            inner: Arc::new(
                ReaderWriterLatch::new(
                    (
                        // Ref count
                        1,
                        Some(
                            UnderlyingPage::new(
                                page_id,
                                data,
                            )
                        )
                    )
                )
            ),
        }
    }

    /// Returns the page id
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use [`Page::try_get_page_id`].
    ///
    /// # Panics
    ///
    /// Panics if the underlying page has weak ref and cant upgrade to a strong one
    ///
    pub fn get_page_id(&self) -> PageId {
        self.try_get_page_id().unwrap()
    }

    pub fn try_get_page_id(&self) -> Option<PageId> {
        /// We don't lock as the data cant be changed in the middle as
        /// 1. it's behind `Arc` so we hold reference to the data, and the underlying data won't be dropped in the middle
        /// 2. We use `ReaderWriterLatch` so no one can change in the middle
        let inner_guard = self.inner.read();

        let (_, underlying) = inner_guard.deref();

        if let Some(p) = underlying {
            return Some(p.get_page_id());
        }

        None
    }

    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> usize {
        let inner_guard = self.inner.read();

        let r = inner_guard.deref();

        r.0
    }

    /// Set page dirty state
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use [`Page::try_set_is_dirty`].
    ///
    /// # Panics
    ///
    /// Panics if the underlying page has weak ref and cant upgrade to a strong one
    ///
    pub fn set_is_dirty(&self, is_dirty: bool) {
        self.try_set_is_dirty(is_dirty).expect("Must be able to set dirty state")
    }

    pub fn try_set_is_dirty(&self, is_dirty: bool) -> Option<()> {
        let mut inner_guard = self.inner.write();

        let (_, underlying) = inner_guard.deref_mut();

        if let Some(p) = underlying {
            p.set_is_dirty(is_dirty);
            return Some(());
        }

        None
    }

    /// Check if page is dirty
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use [`Page::try_is_dirty`].
    ///
    /// # Panics
    ///
    /// Panics if the underlying page has weak ref and cant upgrade to a strong one
    ///
    pub fn is_dirty(&self) -> bool {
        self.try_is_dirty().expect("Must be able to check if dirty")
    }

    pub fn try_is_dirty(&self) -> Option<bool> {
        let inner_guard = self.inner.read();

        let (_, underlying) = inner_guard.deref();

        if let Some(p) = underlying {
            return Some(p.is_dirty());
        }

        None
    }

    pub fn pin(&mut self) -> Option<()> {
        if self.is_pinned {
            // Already pinned
            return Some(());
        }

        let mut inner_guard = self.inner.write();

        let r = inner_guard.deref_mut();

        if r.0 == 0 {
            // Already dropped
            return None;
        }

        inner_guard.0 += 1;
        self.is_pinned = true;

        Some(())
    }

    pub fn unpin(&mut self) {
        if !self.is_pinned {
            // Already unpinned
            return;
        }

        self.is_pinned = false;
        let mut inner_guard = self.inner.write();

        let r = inner_guard.deref_mut();

        r.0 -= 1;

        if r.0 == 0 {
            // TODO - should remove the underlying page on unpin everything?
            r.1 = None
        }
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn is_unpinned(&self) -> bool {
        !self.is_pinned()
    }

    pub fn exists(&self) -> bool {
        let inner_guard = self.inner.read();

        let r = inner_guard.deref();

        r.0 > 0
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        let inner = Arc::clone(&self.inner);

        if self.is_pinned {
            let mut inner_guard = self.inner.write();

            let r = inner_guard.deref_mut();

            // Increase internal ref count
            r.0 += 1;
        }

        Page {
            is_pinned: self.is_pinned,
            inner,
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::new(INVALID_PAGE_ID)
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        let self_guard = self.inner.read();
        let other_guard = other.inner.read();

        let self_ref = self_guard.deref();
        let other_ref = other_guard.deref();

        return self_ref.1.eq(&other_ref.1);
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        // Avoid decreasing count if this page instance is not pinned
        if !self.is_pinned {
            return;
        }

        let mut guard = self.inner.write();

        guard.deref_mut().0 -= 1;

        // If the last ref
        // TODO - should really do that? as when no other refs it will drop automatically
        if guard.deref().0 == 0 {
            guard.deref_mut().1 = None;
        }
    }
}


#[cfg(test)]
mod tests {
    use common::config::BUSTUB_PAGE_SIZE;
    use crate::page::page::Page;

    // #########################
    //         Create
    // #########################

    #[test]
    fn should_create_as_strong_with_only_page_id() {
        let page = Page::new(1);

        assert_eq!(page.exists(), true);
        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.is_unpinned(), false);
        assert_eq!(page.get_pin_count(), 1);
    }

    #[test]
    fn should_create_as_strong_with_page_id_and_data() {
        let page = Page::create_with_data(1, [2; BUSTUB_PAGE_SIZE]);

        assert_eq!(page.exists(), true);
        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.is_unpinned(), false);
        assert_eq!(page.get_pin_count(), 1);
    }

    // #########################
    //        Set is dirty
    // #########################

    #[test]
    fn should_be_able_to_mark_as_dirty_when_have_strong_ref() {
        let mut page = Page::new(1);
        page.pin();

        assert_eq!(page.is_dirty(), false);
        assert_eq!(page.try_is_dirty().unwrap(), false);

        page.set_is_dirty(true);

        assert_eq!(page.is_dirty(), true);
        assert_eq!(page.try_is_dirty().unwrap(), true);
    }

    #[test]
    fn should_be_able_to_mark_as_dirty_when_have_weak_ref() {
        let page_og = Page::new(1);
        // Avoid the value being cleaned up
        let mut page = page_og.clone();
        page.unpin();

        assert_eq!(page.is_dirty(), false);
        assert_eq!(page_og.is_dirty(), false);
        assert_eq!(page.try_is_dirty().unwrap(), false);
        assert_eq!(page_og.try_is_dirty().unwrap(), false);

        // Set as dirty and check that the original and the cloned are in sync
        page.set_is_dirty(true);

        assert_eq!(page.is_dirty(), true);
        assert_eq!(page_og.is_dirty(), true);
        assert_eq!(page.try_is_dirty().unwrap(), true);
        assert_eq!(page_og.try_is_dirty().unwrap(), true);

        // Set as not dirty and check that the original and the cloned are in sync
        page_og.set_is_dirty(false);

        assert_eq!(page_og.is_dirty(), false);
        assert_eq!(page.is_dirty(), false);
        assert_eq!(page_og.try_is_dirty().unwrap(), false);
        assert_eq!(page.try_is_dirty().unwrap(), false);
    }

    #[test]
    fn should_not_be_able_to_set_dirty_state_when_have_weak_ref_with_dropped_value() {
        let mut page = Page::new(1);

        page.unpin();

        assert_eq!(page.try_set_is_dirty(true), None);
        assert_eq!(page.try_set_is_dirty(false), None);
    }

    // ##################
    //        Pin
    // ##################
    #[test]
    fn should_be_able_to_pin_page_with_already_strong_ref() {
        let mut page = Page::new(1);

        page.pin().expect("should be able to pin");

        assert_eq!(page.is_pinned(), true);
    }

    #[test]
    fn should_be_able_to_pin_page_with_existing_weak_ref() {
        let page_og = Page::new(1);
        // Avoid the value being cleaned up
        let mut page = page_og.clone();
        page.unpin();

        assert_eq!(page_og.is_pinned(), true);
        assert_eq!(page.is_pinned(), false);

        page.pin().expect("should be able to pin");

        assert_eq!(page.is_pinned(), true);
    }

    #[test]
    fn should_not_be_able_to_pin_page_with_dropped_weak_ref() {
        let mut page = Page::new(1);
        page.unpin();

        assert_eq!(page.pin(), None, "should not be able to pin when there are no references to existing");
    }

    #[test]
    fn should_keep_pin_count_on_pin_strong() {
        let mut page = Page::new(1);

        let original_pin_count = page.get_pin_count();
        assert_eq!(original_pin_count, 1);

        page.pin().expect("should be able to pin");

        assert_eq!(page.get_pin_count(), original_pin_count);
    }

    #[test]
    fn should_increase_pin_count_on_pin_weak() {
        let mut page = Page::new(1);

        let original_pin_count = page.get_pin_count();
        assert_eq!(original_pin_count, 1);

        page.pin().expect("should be able to pin");

        assert_eq!(page.get_pin_count(), original_pin_count);
    }

    #[test]
    fn should_keep_pin_count_on_pin_weak_dropped() {
        let mut page = Page::new(1);
        page.unpin();

        assert_eq!(page.get_pin_count(), 0);

        assert_eq!(page.pin(), None, "should not be able to pin when there are no references to existing");

        assert_eq!(page.get_pin_count(), 0);
    }

    // ##################
    //        Unpin
    // ##################

    // TODO
    #[ignore]
    #[test]
    fn should_unpin_page_with_strong_ref() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_unpin_page_with_existing_weak_ref() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_unpin_page_with_dropped_weak_ref() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_decrease_pin_count_on_unpin_strong() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_keep_pin_count_on_unpin_weak() {
        todo!()
    }

    // ##################
    //        Clone
    // ##################

    // TODO
    #[ignore]
    #[test]
    fn clone_on_strong_should_increase_pin_count() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_strong_should_keep_strong() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_strong_should_keep_dirty_status() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_strong_should_keep_lock() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_strong_should_sync_dirty_status_changes() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_strong_should_sync_data_changes() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_weak_should_keep_the_pin_count() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_weak_should_keep_weak() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_weak_should_keep_dirty_status() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_weak_should_keep_lock() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_weak_should_sync_dirty_status_changes() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn clone_on_weak_should_sync_data_changes() {
        todo!()
    }
}
