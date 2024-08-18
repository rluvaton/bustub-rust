use crate::page::underlying_page::UnderlyingPage;
use common::config::{PageData, PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use common::ReaderWriterLatch;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);

/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug)]
pub struct Page {
    inner: Arc<ReaderWriterLatch<UnderlyingPage>>,
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
        let mut inner = Arc::new(
            ReaderWriterLatch::new(
                UnderlyingPage::new(
                    page_id,
                    data,
                )
            )
        );

        let ptr = Arc::into_raw(inner);

        unsafe {
            Arc::increment_strong_count(ptr);
            inner = Arc::from_raw(ptr);
        }

        Page {
            // Starting as pinned
            is_pinned: true,
            inner,
        }
    }

    /// Returns the page id
    pub fn get_page_id(&self) -> PageId {
        let inner_guard = self.inner.read();

        let underlying = inner_guard.deref();

        underlying.get_page_id()
    }

    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> usize {
        // This can never be 0
        // Ignore own + 1 we did in the new function
        Arc::strong_count(&self.inner) - 1
    }

    pub fn pin(&mut self) {
        if self.is_pinned {
            // Already pinned
            return;
        }

        {
            let cloned_arc = Arc::clone(&self.inner);

            let ptr = Arc::into_raw(cloned_arc);
            unsafe {
                self.inner = Arc::from_raw(ptr);

                // Increment by 1 as we pinned
                Arc::increment_strong_count(ptr)
            }
        }

        self.is_pinned = true;
    }

    pub fn unpin(&mut self) {
        if !self.is_pinned {
            // Already unpinned
            return;
        }

        {
            let cloned_arc = Arc::clone(&self.inner);

            let ptr = Arc::into_raw(cloned_arc);
            unsafe {
                self.inner = Arc::from_raw(ptr);

                // Decrement by 1 as we unpinned
                Arc::decrement_strong_count(ptr)
            }
        }

        self.is_pinned = false;
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn is_unpinned(&self) -> bool {
        !self.is_pinned()
    }

    /// Run function with read lock and get the underlying page
    ///
    /// # Arguments
    ///
    /// * `with_read_lock`: function to run with `read` lock that get the underlying page
    ///
    /// returns: R `with_read_lock` return value
    ///
    /// # Examples
    /// ```
    /// use storage::Page;
    ///
    /// let page = Page::new(1);
    /// page.with_read(|u| {
    ///     assert_eq!(u.is_dirty(), false);
    /// });
    /// ```
    pub fn with_read<F: FnOnce(&UnderlyingPage) -> R, R>(&self, with_read_lock: F) -> R {
        let mut inner_guard = self.inner.read();

        with_read_lock(inner_guard.deref())
    }

    /// Run function with write lock and get the underlying page
    ///
    /// # Arguments
    ///
    /// * `with_write_lock`: function to run with write lock that get the underlying page
    ///
    /// returns: R `with_write_lock` return value
    pub fn with_write<F: FnOnce(&mut UnderlyingPage) -> R, R>(&self, with_write_lock: F) -> R {
        let mut inner_guard = self.inner.write();

        with_write_lock(inner_guard.deref_mut())
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        let mut inner = Arc::clone(&self.inner);

        // TODO - how to detect if self is already locking ? to detect dead locks
        // TODO - can have other deadlocks when locking to increase the pin count

        // TODO - increase pin count using the unsafe?

        // If not pinned should decrease pin count
        if !self.is_pinned {
            assert!(Arc::strong_count(&inner) > 1, "Must clone Page with inner ref count greater than 1");

            // Clone and decrement strong count manually
            let ptr = Arc::into_raw(inner);
            unsafe {
                inner = Arc::from_raw(ptr);
                Arc::decrement_strong_count(ptr)
            }
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

        return self_ref.eq(&other_ref);
    }
}

impl Drop for Page {
    fn drop(&mut self) {
        // Avoid decreasing count if this page instance is not pinned
        if !self.is_pinned {
            return;
        }

        // If the last one (as we create another one at creation)
        // then manually decrease
        if Arc::strong_count(&self.inner) == 2 {
            // No need to decrease strong count twice as we gonna replace the self.inner value which will replace the old ref with new ref
            let cloned_arc = Arc::clone(&self.inner);

            let ptr = Arc::into_raw(cloned_arc);
            unsafe {
                self.inner = Arc::from_raw(ptr);

                // Change it back to 1 and after this drop function will finish the original strong ref will be cleaned up as well
                Arc::decrement_strong_count(ptr);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::page::page::Page;
    use common::config::BUSTUB_PAGE_SIZE;
    use parking_lot::Mutex;
    use std::fmt::{Debug, Display};
    use std::sync::Arc;
    use std::time::Duration;
    use std::{panic, thread};
    use std::ops::Deref;
    use std::panic::UnwindSafe;

    fn run_test_in_separate_thread<F: FnOnce() -> () + UnwindSafe + Send + 'static>(test: F, test_time: u64) {
        let panic_info_container: Arc<Mutex<Option<thread::Result<()>>>> = Arc::new(Mutex::new(None));

        let panic_info_thread_instance = Arc::clone(&panic_info_container);
        let t = thread::spawn(move || {
            let res = panic::catch_unwind(test);

            panic_info_thread_instance.lock().get_or_insert(res);
        });

        thread::sleep(Duration::from_millis(test_time));

        assert_eq!(t.is_finished(), true, "Should not have deadlock");

        {
            // Get the panic info, lock for 100ms in case the lock can't be unlocked
            let panic_info_guard = panic_info_container
                .try_lock_for(Duration::from_millis(100))
                .expect("Should be able to lock");


            // In case the thread finished check that no error exists (assertion inside)
            let val = panic_info_guard.deref();

            if let Some(res) = val {
                if res.is_err() {
                    println!("throw: {:#?}", res);
                    assert!(false, "should not throw inside spawned thread")
                }
            }
        }
    }

    // #########################
    //         Create
    // #########################

    #[test]
    fn should_create_as_pinned_with_only_page_id() {
        let page = Page::new(1);

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.is_unpinned(), false);
        assert_eq!(page.get_pin_count(), 1);
    }

    #[test]
    fn should_create_as_pinned_with_page_id_and_data() {
        let page = Page::create_with_data(1, [2; BUSTUB_PAGE_SIZE]);

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.is_unpinned(), false);
        assert_eq!(page.get_pin_count(), 1);
    }

    #[test]
    fn should_create_with_2_refs_with_only_page_id() {
        let page = Page::new(1);

        assert_eq!(Arc::strong_count(&page.inner), 2);
    }

    #[test]
    fn should_create_with_2_refs_with_page_id_and_data() {
        let page = Page::create_with_data(1, [2; BUSTUB_PAGE_SIZE]);

        assert_eq!(Arc::strong_count(&page.inner), 2);
    }

    // #########################
    //        Set is dirty
    // #########################

    #[test]
    fn should_be_able_to_mark_as_dirty_when_pinned() {
        let mut page = Page::new(1);
        page.pin();

        assert_eq!(page.with_read(|u| u.is_dirty()), false);

        page.with_write(|u| u.set_is_dirty(true));

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn should_be_able_to_mark_as_dirty_when_unpinned_page() {
        let page_og = Page::new(1);
        // Avoid the value being cleaned up
        let mut page = page_og.clone();
        page.unpin();

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        assert_eq!(page_og.with_read(|u| u.is_dirty()), false);

        // Set as dirty and check that the original and the cloned are in sync
        page.with_write(|u| u.set_is_dirty(true));

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(page_og.with_read(|u| u.is_dirty()), true);

        // Set as not dirty and check that the original and the cloned are in sync
        page_og.with_write(|u| u.set_is_dirty(false));

        assert_eq!(page_og.with_read(|u| u.is_dirty()), false);
        assert_eq!(page.with_read(|u| u.is_dirty()), false);
    }

    #[test]
    fn should_be_able_to_set_dirty_state_on_unpinned_page_last_ref() {
        let mut page = Page::new(1);

        page.unpin();

        page.with_write(|u| u.set_is_dirty(true));

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
    }

    // ##################
    //        Pin
    // ##################
    #[test]
    fn should_be_able_to_pin_page_with_already_pinned() {
        let mut page = Page::new(1);

        page.pin();

        assert_eq!(page.is_pinned(), true);
    }

    #[test]
    fn should_be_able_to_pin_page_with_unpinned_page() {
        let page_og = Page::new(1);
        // Avoid the value being cleaned up
        let mut page = page_og.clone();
        page.unpin();

        assert_eq!(page_og.is_pinned(), true);
        assert_eq!(page.is_pinned(), false);

        page.pin();

        assert_eq!(page.is_pinned(), true);
    }

    #[test]
    fn should_be_able_to_pin_page_that_has_no_pinned_refs() {
        let mut page = Page::new(1);
        page.unpin();

        page.pin();

        assert_eq!(page.is_pinned(), true);
    }

    #[test]
    fn should_keep_pin_count_on_pinned() {
        let mut page = Page::new(1);

        let original_pin_count = page.get_pin_count();
        assert_eq!(original_pin_count, 1);

        page.pin();

        assert_eq!(page.get_pin_count(), original_pin_count);
    }

    #[test]
    fn should_increase_pin_count_on_pin_unpinned_page() {
        let mut page = Page::new(1);

        let original_pin_count = page.get_pin_count();
        assert_eq!(original_pin_count, 1);

        page.pin();

        assert_eq!(page.get_pin_count(), original_pin_count);
    }

    #[test]
    fn should_increase_pin_count_on_pin_last_unpinned_page() {
        let mut page = Page::new(1);
        page.unpin();

        assert_eq!(page.get_pin_count(), 0);

        page.pin();

        assert_eq!(page.get_pin_count(), 1);
    }

    // ##################
    //        Unpin
    // ##################

    #[test]
    fn should_unpin_pinned_page() {
        let mut page = Page::new(1);

        assert_eq!(page.is_pinned(), true);

        page.unpin();

        assert_eq!(page.is_pinned(), false);
    }

    #[test]
    fn should_unpin_page_with_existing_unpinned_page() {
        let page_og = Page::new(1);
        // Avoid the value being cleaned up
        let mut page = page_og.clone();
        page.unpin();

        assert_eq!(page.is_pinned(), false);

        page.unpin();

        assert_eq!(page.is_pinned(), false);
    }

    #[test]
    fn should_unpin_last_unpinned_page() {
        let mut page = Page::new(1);
        page.unpin();

        assert_eq!(page.is_pinned(), false);

        page.unpin();

        assert_eq!(page.is_pinned(), false);
    }

    #[test]
    fn should_decrease_pin_count_on_unpin_pinned_page() {
        let mut page = Page::new(1);
        let mut other_page = page.clone();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 2);
        assert_eq!(other_page.is_pinned(), true);
        assert_eq!(other_page.get_pin_count(), 2);

        other_page.unpin();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);

        page.unpin();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(other_page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);
        assert_eq!(other_page.get_pin_count(), 0);
    }

    #[test]
    fn should_keep_pin_count_on_unpin_unpinned_page() {
        let mut page = Page::new(1);
        let mut other_page = page.clone();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 2);
        assert_eq!(other_page.is_pinned(), true);
        assert_eq!(other_page.get_pin_count(), 2);

        other_page.unpin();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);

        // Unpin existing unpinned
        other_page.unpin();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);
    }

    #[test]
    fn should_not_fail_to_unpin_last_pinned_page() {
        let mut page = Page::new(1);

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 1);

        // Unpin multiple times
        page.unpin();
        page.unpin();
        page.unpin();
        page.unpin();
        page.unpin();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);
    }

    #[test]
    fn should_return_0_for_pin_count_when_no_page_has_any_pin_left() {
        let mut page = Page::new(1);

        page.unpin();

        assert_eq!(page.get_pin_count(), 0);
    }

    // ##################
    //        Clone
    // ##################

    #[test]
    fn clone_on_pinned_should_increase_pin_count() {
        let mut page = Page::new(1);
        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 1);

        let mut other_page = page.clone();

        assert_eq!(page.get_pin_count(), 2);
        assert_eq!(other_page.get_pin_count(), 2);
    }

    #[test]
    fn clone_on_pinned_should_keep_pinned() {
        let mut page = Page::new(1);
        assert_eq!(page.is_pinned(), true);

        let mut other_page = page.clone();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), true);
    }

    #[test]
    fn clone_on_pinned_should_keep_dirty_status() {
        let mut page = Page::new(1);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        page.with_write(|u| u.set_is_dirty(true));
        assert_eq!(page.with_read(|u| u.is_dirty()), true);

        assert_eq!(page.is_pinned(), true);

        let other_page = page.clone();

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn clone_on_pinned_should_keep_lock_state() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);

                assert_eq!(page.inner.is_locked(), false);
                let guard = page.inner.read();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_on_pinned_with_read_lock_acquired_should_not_cause_deadlocks() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);

                assert_eq!(page.inner.is_locked(), false);
                let guard = page.inner.read();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_on_pinned_with_write_lock_acquired_should_not_cause_deadlocks() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);

                assert_eq!(page.inner.is_locked(), false);
                let guard = page.inner.write();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_on_pinned_should_sync_dirty_status_changes() {
        let mut page = Page::new(1);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);

        assert_eq!(page.is_pinned(), true);

        let other_page = page.clone();

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), false);

        page.with_write(|u| u.set_is_dirty(true));
        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn clone_on_pinned_should_sync_lock_changes() {
        let mut page = Page::new(1);
        let other_page = page.clone();

        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);

        {
            let guard = page.inner.read();

            assert_eq!(page.inner.is_locked(), true);
            assert_eq!(other_page.inner.is_locked(), true);
        }

        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);

        {
            let guard = other_page.inner.read();

            assert_eq!(page.inner.is_locked(), true);
            assert_eq!(other_page.inner.is_locked(), true);
        }


        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);
    }

    #[test]
    fn clone_on_unpinned_page_should_keep_the_pin_count() {
        let mut page = Page::new(1);
        page.unpin();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);

        let other_unpinned = page.clone();

        assert_eq!(page.get_pin_count(), 0);
        assert_eq!(other_unpinned.get_pin_count(), 0);
    }

    #[test]
    fn clone_on_unpinned_page_should_keep_unpinned() {
        let mut page = Page::new(1);
        page.unpin();

        assert_eq!(page.is_pinned(), false);

        let other_unpinned = page.clone();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(other_unpinned.is_pinned(), false);
    }

    #[test]
    fn clone_on_unpinned_page_should_keep_dirty_status() {
        let mut page = Page::new(1);

        page.unpin();
        assert_eq!(page.is_pinned(), false);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        page.with_write(|u| u.set_is_dirty(true));
        assert_eq!(page.with_read(|u| u.is_dirty()), true);

        let other_unpinned = page.clone();

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(other_unpinned.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn clone_on_unpinned_should_keep_lock_state() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);
                page.unpin();

                assert_eq!(page.inner.is_locked(), false);
                let guard = page.inner.read();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_on_unpinned_with_read_lock_acquired_should_not_cause_deadlocks() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);
                page.unpin();

                assert_eq!(page.inner.is_locked(), false);
                let guard = page.inner.read();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_on_unpinned_with_write_lock_acquired_should_not_cause_deadlocks() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);
                page.unpin();

                assert_eq!(page.inner.is_locked(), false);
                let guard = page.inner.write();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_on_unpinned_page_should_sync_dirty_status_changes() {
        let mut page = Page::new(1);
        page.unpin();
        assert_eq!(page.is_pinned(), false);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);

        let other_page = page.clone();

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), false);

        page.with_write(|u| u.set_is_dirty(true));
        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn clone_on_unpinned_page_should_sync_lock_changes() {
        let mut page = Page::new(1);
        page.unpin();

        let other_page = page.clone();

        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);

        {
            let guard = page.inner.read();

            assert_eq!(page.inner.is_locked(), true);
            assert_eq!(other_page.inner.is_locked(), true);
        }

        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);

        {
            let guard = other_page.inner.read();

            assert_eq!(page.inner.is_locked(), true);
            assert_eq!(other_page.inner.is_locked(), true);
        }


        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);
    }

    // ##################
    //     with_read
    // ##################

    #[test]
    fn with_read_should_acquire_read_lock() {
        let page = Page::new(1);

        page.with_read(|_| {
            assert_eq!(page.inner.is_locked(), true, "should be locked");
            assert_eq!(page.inner.is_locked_exclusive(), false, "Should not be locked in write mode");
        });
    }

    #[test]
    fn with_read_should_release_lock_after_scope() {
        let page = Page::new(1);

        page.with_read(|_| {});

        assert_eq!(page.inner.is_locked(), false, "should not be locked");
    }

    #[test]
    fn with_read_should_get_the_underlying_page() {
        let page = Page::new(1);

        page.with_read(|u| {
            assert_eq!(u.get_page_id(), 1);
        });
    }

    #[test]
    fn with_read_should_return_the_function_return_value() {
        let page = Page::new(1);


        let res = page.with_read(|u| {
            5
        });

        assert_eq!(res, 5);
    }

    // ##################
    //     with_write
    // ##################

    #[test]
    fn with_write_should_acquire_write_lock() {
        let page = Page::new(1);

        page.with_write(|_| {
            assert_eq!(page.inner.is_locked(), true, "should be locked");
            assert_eq!(page.inner.is_locked_exclusive(), true, "Should be locked in write mode");
        });
    }

    #[test]
    fn with_write_should_release_lock_after_scope() {
        let page = Page::new(1);

        page.with_write(|_| {});

        assert_eq!(page.inner.is_locked(), false, "should not be locked");
    }

    #[test]
    fn with_write_should_get_the_underlying_page() {
        let page = Page::new(1);

        page.with_write(|u| {
            assert_eq!(u.get_page_id(), 1);
        });
    }

    #[test]
    fn with_write_should_be_able_to_mutate_inner_page() {
        let page = Page::new(1);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);

        page.with_write(|u| {
            u.set_is_dirty(true)
        });

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn with_write_should_return_the_function_return_value() {
        let page = Page::new(1);


        let res = page.with_write(|u| {
            5
        });

        assert_eq!(res, 5);
    }
}
