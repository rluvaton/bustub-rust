use crate::page::underlying_page::UnderlyingPage;
use common::config::{PageData, PageId, BUSTUB_PAGE_SIZE, INVALID_PAGE_ID};
use common::ReaderWriterLatch;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use log::warn;
use tracy_client::span;
// static_assert(sizeof(page_id_t) == 4);
// static_assert(sizeof(lsn_t) == 4);


// While waiting - red-ish (darker than page lock)
const PAGE_LOCK_WAITING_COLOR: u32 = 0xCC0000;

// While holding the page lock - green-ish (darker than page lock)
const PAGE_LOCK_HOLDING_COLOR: u32 = 0x006A4E;

/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug)]
pub struct Page {
    inner: Arc<ReaderWriterLatch<UnderlyingPage>>,
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
            inner: Arc::new(
                ReaderWriterLatch::new(
                    UnderlyingPage::new(
                        page_id,
                        data,
                    )
                )
            ),
        }
    }


    /** @return the pin count of this page */
    pub fn get_pin_count(&self) -> usize {
        self.with_read(
            #[inline(always)]
            |u| u.get_pin_count()
        )
    }

    /// Pin page and return the current number of pins
    ///
    /// # Safety
    /// Calling pin multiple times can increase the pin counter more than needed
    ///
    /// only pin once per thread
    pub fn pin(&mut self) -> usize {
        self.with_write(
            #[inline(always)]
            |u| {
            let mut p = u.get_pin_count();

            p += 1;
            let ref_count = Arc::strong_count(&self.inner);

            if p > ref_count {
                warn!("Got more pins than references to the page, this is probably a mistake");

                // Cant be more than there are references to the page
                p = ref_count;
            }

            u.set_pin_count(p);

            p
        })
    }


    /// Unpin page and return the current number of pins
    ///
    /// # Safety
    /// Calling unpin multiple times can decrease the pin counter more than needed
    ///
    /// only unpin after each pin
    pub fn unpin(&mut self) -> usize {
        self.with_write(
            #[inline(always)]
            |u| {
            let mut p = u.get_pin_count();

            if p == 0 {
                warn!("Trying to unpin page which has no pins this is a mistake");
                return 0;
            }

            p -= 1;

            u.set_pin_count(p);

            p
        })
    }

    pub fn is_pinned(&self) -> bool {
        self.get_pin_count() > 0
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
    /// let page_id = page.with_read(|u| u.get_page_id());
    ///
    /// page.with_read(|u| {
    ///     assert_eq!(u.is_dirty(), false);
    /// });
    /// ```
    #[inline(always)]
    pub fn with_read<F: FnOnce(&UnderlyingPage) -> R, R>(&self, with_read_lock: F) -> R {
        let acquiring_page_latch = span!("Acquiring read page latch");

        // Red color while waiting
        acquiring_page_latch.emit_color(PAGE_LOCK_WAITING_COLOR);
        let  inner_guard = self.inner.read();

        drop(acquiring_page_latch);

        let holding_page_latch = span!("Holding read page latch");

        // Green color while holding
        holding_page_latch.emit_color(PAGE_LOCK_HOLDING_COLOR);

        with_read_lock(inner_guard.deref())
    }

    /// Run function with write lock and get the underlying page
    ///
    /// # Arguments
    ///
    /// * `with_write_lock`: function to run with write lock that get the underlying page
    ///
    /// returns: R `with_write_lock` return value
    ///
    /// # Examples
    /// ```
    /// use storage::Page;
    ///
    /// let page = Page::new(1);
    ///
    /// page.with_write(|u| {
    ///     u.set_is_dirty(true);
    /// });
    /// ```
    #[inline(always)]
    pub fn with_write<F: FnOnce(&mut UnderlyingPage) -> R, R>(&self, with_write_lock: F) -> R {

        let acquiring_page_latch = span!("Acquiring write page latch");

        // Red color while waiting
        acquiring_page_latch.emit_color(PAGE_LOCK_WAITING_COLOR);
        let mut inner_guard = self.inner.write();

        drop(acquiring_page_latch);

        let holding_page_latch = span!("Holding write page latch");

        // Green color while holding
        holding_page_latch.emit_color(PAGE_LOCK_HOLDING_COLOR);

        with_write_lock(inner_guard.deref_mut())
    }
}

impl Clone for Page {
    fn clone(&self) -> Self {
        let inner = Arc::clone(&self.inner);

        let page = Page {
            inner,
        };

        // Clone should not increase pin, this should be done by the consumer of the page

        page
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
        // Drop reference does not change a pin
    }
}


#[cfg(test)]
mod tests {
    use crate::page::page::Page;
    use parking_lot::Mutex;
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
        let mut page = Page::new(1);
        page.unpin();

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
    fn should_increase_pin_count_on_pin_unpinned_page() {
        let mut page = Page::new(1);

        let original_pin_count = page.get_pin_count();
        assert_eq!(original_pin_count, 0);

        page.pin();

        assert_eq!(page.get_pin_count(), 1);
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
        page.pin();

        assert_eq!(page.is_pinned(), true);

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
        page.pin();
        let mut other_page = page.clone();
        other_page.pin();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 2);
        assert_eq!(other_page.is_pinned(), true);
        assert_eq!(other_page.get_pin_count(), 2);

        other_page.unpin();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);

        page.unpin();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(other_page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);
        assert_eq!(other_page.get_pin_count(), 0);
    }

    // TODO - should fix this test
    #[ignore]
    #[test]
    fn should_keep_pin_count_on_unpin_unpinned_page() {
        let page = Page::new(1);
        let mut other_page = page.clone();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 2);
        assert_eq!(other_page.is_pinned(), true);
        assert_eq!(other_page.get_pin_count(), 2);

        other_page.unpin();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);

        // Unpin existing unpinned
        other_page.unpin();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(other_page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);
    }

    #[test]
    fn should_not_fail_to_unpin_last_pinned_page() {
        let mut page = Page::new(1);
        page.pin();

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
    fn clone_on_pinned_should_keep_pin_count() {
        let mut page = Page::new(1);
        assert_eq!(page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);

        page.pin();
        assert_eq!(page.is_pinned(), true);
        assert_eq!(page.get_pin_count(), 1);


        let other_page = page.clone();

        assert_eq!(page.get_pin_count(), 1);
        assert_eq!(other_page.get_pin_count(), 1);
    }

    #[test]
    fn unpin_with_clone_should_not_crash() {
        let page: Page;
        {
            let mut new_page = Page::new(1);

            assert_eq!(new_page.is_pinned(), false);
            assert_eq!(new_page.get_pin_count(), 0);

            new_page.unpin();
            page = new_page.clone();
        }

        assert_eq!(page.get_pin_count(), 0);
    }

    #[test]
    fn clone_on_pinned_should_keep_pinned() {
        let mut page = Page::new(1);
        assert_eq!(page.is_pinned(), false);

        page.pin();
        assert_eq!(page.is_pinned(), true);

        let other_page = page.clone();

        assert_eq!(page.is_pinned(), true);
        assert_eq!(other_page.is_pinned(), true);
    }

    #[test]
    fn clone_on_pinned_should_keep_dirty_status() {
        let mut page = Page::new(1);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        page.with_write(|u| u.set_is_dirty(true));
        assert_eq!(page.with_read(|u| u.is_dirty()), true);

        page.pin();
        assert_eq!(page.is_pinned(), true);

        let other_page = page.clone();

        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn clone_should_keep_lock_state() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let page = Page::new(1);

                assert_eq!(page.inner.is_locked(), false);
                let _guard = page.inner.read();
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
                page.pin();

                assert_eq!(page.inner.is_locked(), false);
                let _guard = page.inner.read();
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
                page.pin();

                assert_eq!(page.inner.is_locked(), false);
                let _guard = page.inner.write();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
    }

    #[test]
    fn clone_should_sync_dirty_status_changes() {
        let page = Page::new(1);

        assert_eq!(page.with_read(|u| u.is_dirty()), false);

        let other_page = page.clone();

        assert_eq!(page.with_read(|u| u.is_dirty()), false);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), false);

        page.with_write(|u| u.set_is_dirty(true));
        assert_eq!(page.with_read(|u| u.is_dirty()), true);
        assert_eq!(other_page.with_read(|u| u.is_dirty()), true);
    }

    #[test]
    fn clone_should_sync_lock_changes() {
        let page = Page::new(1);
        let other_page = page.clone();

        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);

        {
            let _guard = page.inner.read();

            assert_eq!(page.inner.is_locked(), true);
            assert_eq!(other_page.inner.is_locked(), true);
        }

        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);

        {
            let _guard = other_page.inner.read();

            assert_eq!(page.inner.is_locked(), true);
            assert_eq!(other_page.inner.is_locked(), true);
        }


        assert_eq!(page.inner.is_locked(), false);
        assert_eq!(other_page.inner.is_locked(), false);
    }

    #[test]
    fn clone_on_unpinned_page_should_keep_pin_count() {
        let page = Page::new(1);

        assert_eq!(page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);

        let cloned_pinned = page.clone();

        assert_eq!(page.is_pinned(), false);
        assert_eq!(page.get_pin_count(), 0);
        assert_eq!(cloned_pinned.is_pinned(), false);
        assert_eq!(cloned_pinned.get_pin_count(), 0);
    }

    #[test]
    fn clone_on_unpinned_with_read_lock_acquired_should_not_cause_deadlocks() {
        // Running in separate thread to avoid pausing the current thread in case of a deadlock
        run_test_in_separate_thread(
            || {
                let mut page = Page::new(1);
                page.unpin();

                assert_eq!(page.inner.is_locked(), false);
                let _guard = page.inner.read();
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
                let _guard = page.inner.write();
                assert_eq!(page.inner.is_locked(), true);

                let other_page = page.clone();

                assert_eq!(other_page.inner.is_locked(), true)
            },
            // 200ms for the test is more than enough for the lock to finish
            200,
        )
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


        let res = page.with_read(|_| {
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


        let res = page.with_write(|_| {
            5
        });

        assert_eq!(res, 5);
    }
}
