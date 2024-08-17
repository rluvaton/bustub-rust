use std::ops::{Deref, DerefMut};
use crate::page::underlying_page::UnderlyingPage;
use common::config::{PageData, PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use common::ReaderWriterLatch;
use std::sync::{Arc, Weak};
use anyhow::anyhow;
// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);

#[derive(Debug)]
enum PageRef {
    Strong(UnderlyingPage),
    Weak(UnderlyingPage),
}

/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug, Clone)]
pub struct Page {
    // TODO - wrap this with arc? as this is not really the data
    inner: Arc<ReaderWriterLatch<(usize, PageRef)>>,
    //
    // ref_count: usize
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
            // ref_count: 1,
            inner: Arc::new(
                ReaderWriterLatch::new((
                    // Self
                    1,
                    PageRef::Strong(
                        // Arc::new(
                        UnderlyingPage::new(
                            page_id,
                            data,
                        )
                        // )
                    )
                )
                )
            )
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
        let inner_guard = self.inner.read();

        let r = inner_guard.deref();

        match &r.1 {
            PageRef::Strong(underlying) => {
                Some(underlying.get_page_id())
            }
            PageRef::Weak(w) => {
                // Some(w.upgrade()?.get_page_id())
                Some(w.get_page_id())
            }
        }
    }

    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> usize {
        let inner_guard = self.inner.read();

        let r = inner_guard.deref();

        return r.0;

        // match &r.1 {
        //     PageRef::Strong(underlying) => {
        //         Arc::strong_count(&underlying)
        //     }
        //     PageRef::Weak(w) => {
        //         Weak::strong_count(&w)
        //     }
        // }
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

        let r = inner_guard.deref_mut();

        match &mut r.1 {
            PageRef::Strong(underlying) => {
                // Arc::get_mut(underlying)
                //
                //     // This is probably not true as there are multiple refs
                //     .expect("must be able to get mut ref as no other ref currently hold the lock")
                //     .set_is_dirty(is_dirty);
                underlying
                    .set_is_dirty(is_dirty);

                Some(())
            }
            PageRef::Weak(underlying) => {
                // Arc::get_mut(
                //     &mut underlying.upgrade()?
                // )
                //     // This is probably not true as there are multiple refs
                //     .expect("must be able to get mut ref as no other ref currently hold the lock")
                //     .set_is_dirty(is_dirty);

                underlying
                    .set_is_dirty(is_dirty);

                Some(())
            }
        }
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

        let r = inner_guard.deref();

        match &r.1 {
            PageRef::Strong(underlying) => {
                Some(underlying.is_dirty())
            }
            PageRef::Weak(underlying) => {
                // Some(underlying.upgrade()?.is_dirty())
                Some(underlying.is_dirty())
            }
        }
    }

    pub fn pin(&mut self) -> Option<()> {
        let mut inner_guard = self.inner.write();

        let r = inner_guard.deref_mut();

        if r.0 == 0 {
            // Already dropped
            return None;
        }

        match &r.1 {
            PageRef::Strong(_) => {
                // Already pinned
                Some(())
            }
            PageRef::Weak(w) => {
                inner_guard.0 += 1;
                // let upgrade_res = w.upgrade();
                //
                // if upgrade_res.is_none() {
                //     // Was not able to make the pin
                //     return None;
                // }
                //
                // *inner_guard = (inner_guard.deref().0, PageRef::Strong(upgrade_res.expect("Must exist")));
                //
                // Pinned
                Some(())
            }
        }
    }

    pub fn unpin(&mut self) {
        let mut inner_guard = self.inner.write();

        let r = inner_guard.deref_mut();

        if r.0 == 0 {
            return;
        }

        match &r.1 {
            PageRef::Strong(s) => {
                r.0 -= 1;
                // *inner_guard = (r.0 - 1, PageRef::Weak(Arc::downgrade(&s)));
            }
            PageRef::Weak(_) => {
                // Already unpinned
            }
        }
    }

    pub fn is_pinned(&self) -> bool {
        let inner_guard = self.inner.read();

        let r = inner_guard.deref();

        match &r.1 {
            PageRef::Strong(_) => {
                true
            }
            PageRef::Weak(_) => {
                false
            }
        }
    }

    pub fn is_unpinned(&self) -> bool {
        !self.is_pinned()
    }

    pub fn exists(&self) -> bool {
        let inner_guard = self.inner.read();

        let r = inner_guard.deref();

        return r.0 > 0;

        // match &r.1 {
        //     PageRef::Strong(_) => {
        //         true
        //     }
        //     PageRef::Weak(w) => {
        //         w.upgrade().is_some()
        //     }
        // }
    }

    fn eq_both_weak(a: &Weak<UnderlyingPage>, b: &Weak<UnderlyingPage>) -> bool {
        let self_strong = a.upgrade();
        let other_strong = b.upgrade();

        if self_strong.is_some() && other_strong.is_some() {
            let self_underlying = self_strong.unwrap();
            let other_underlying = other_strong.unwrap();

            return self_underlying.eq(&other_underlying);
        }

        self_strong.is_none() == other_strong.is_none()
    }

    fn eq_combined(strong: &Arc<UnderlyingPage>, weak: &Weak<UnderlyingPage>) -> bool {
        let weak_as_strong = weak.upgrade();

        if weak_as_strong.is_none() {
            return false;
        }

        strong.eq(&weak_as_strong.unwrap())
    }
}

// impl Clone for Page {
//     fn clone(&self) -> Self {
//         // TODO - should clone the lock while also update the ref underlying
//         Page {
//             // ref_count: 1,
//             inner: Arc::clone(&self.inner),
//         }
//         // let inner_guard = self.inner.read();
//         //
//         // match inner_guard.deref() {
//         //     PageRef::Strong(underlying) => {
//         //         Some(underlying.get_page_id())
//         //     }
//         //     PageRef::Weak(w) => {
//         //         Some(w.upgrade()?.get_page_id())
//         //     }
//         // }
//     }
// }

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

        // return self_ref.1.eq(&other_ref.1);

        match &self_ref.1 {
            PageRef::Strong(self_strong) => {
                match &other_ref.1 {
                    PageRef::Strong(other_strong) => {
                        // Both strong
                        self_strong.eq(other_strong)
                    }

                    PageRef::Weak(other_weak) => {
                        // Strong and weak
                        self_strong.eq(other_weak)
                        // Page::eq_combined(self_strong, other_weak)
                    }
                }
            }
            PageRef::Weak(self_weak) => {
                match &other_ref.1 {
                    PageRef::Strong(other_strong) => {
                        self_weak.eq(other_strong)
                        // Weak and strong
                        // Page::eq_combined(other_strong, self_weak)
                    }
                    PageRef::Weak(other_weak) => {
                        self_weak.eq(other_weak)

                        // Both weak
                        // Page::eq_both_weak(self_weak, other_weak)
                    }
                }
            }
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
        // Avoid the being dropped
        let mut page = page_og.clone();
        page.unpin();

        assert_eq!(page.is_dirty(), false);
        assert_eq!(page_og.is_dirty(), false);
        assert_eq!(page.try_is_dirty().unwrap(), false);
        assert_eq!(page_og.try_is_dirty().unwrap(), false);

        page.set_is_dirty(true);

        assert_eq!(page.is_dirty(), true);
        assert_eq!(page_og.is_dirty(), true);
        assert_eq!(page.try_is_dirty().unwrap(), true);
        assert_eq!(page_og.try_is_dirty().unwrap(), true);

        page_og.set_is_dirty(false);

        assert_eq!(page_og.is_dirty(), false);
        assert_eq!(page.is_dirty(), false);
        assert_eq!(page_og.try_is_dirty().unwrap(), false);
        assert_eq!(page.try_is_dirty().unwrap(), false);
    }

    // TODO
    #[ignore]
    #[test]
    fn should_not_be_able_to_mark_as_dirty_when_have_weak_ref_with_dropped_value() {
        todo!()
    }

    // ##################
    //        Pin
    // ##################
    // TODO
    #[ignore]
    #[test]
    fn should_be_able_to_pin_page_with_already_strong_ref() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_be_able_to_pin_page_with_existing_weak_ref() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_not_be_able_to_pin_page_with_dropped_weak_ref() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_increase_pin_count_on_pin_weak() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_keep_pin_count_on_pin_strong() {
        todo!()
    }

    // TODO
    #[ignore]
    #[test]
    fn should_keep_pin_count_on_pin_weak_dropped() {
        todo!()
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
