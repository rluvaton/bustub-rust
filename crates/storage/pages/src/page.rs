use crate::{PageData, PageId, UnderlyingPage, INVALID_PAGE_ID, PAGE_SIZE};
use common::ReaderWriterLatch;
use parking_lot::{RwLockReadGuard, RwLockWriteGuard};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub type PageReadGuard<'a> = RwLockReadGuard<'a, UnderlyingPage>;
pub type PageWriteGuard<'a> = RwLockWriteGuard<'a, UnderlyingPage>;

/**
 * Page is the basic unit of storage within the database system. Page provides a wrapper for actual data pages being
 * held in main memory. Page also contains book-keeping information that is used by the buffer pool manager, e.g.
 * pin count, dirty flag, page id, etc.
 */
#[derive(Debug, Clone)]
pub struct Page(Arc<PageSend>);

#[derive(Debug)]
struct PageSend {
    inner: ReaderWriterLatch<UnderlyingPage>,

    // The pin count of this page.
    pin_count: AtomicIsize,


    // True if the page is dirty, i.e. it is different from its corresponding page on disk.
    is_dirty: AtomicBool,
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
        let page = Self::create_with_data(page_id, [0u8; PAGE_SIZE]);

        // Creating new page is marked as dirty by default in order for new pages to be flushed to disk
        page.0.is_dirty.store(true, Ordering::Relaxed);

        page
    }

    pub fn create_with_data(page_id: PageId, data: PageData) -> Self {
        Page(Arc::new(PageSend {
            inner:
                ReaderWriterLatch::new(
                    UnderlyingPage::new(
                        page_id,
                        data,
                    )
                ),
            pin_count: AtomicIsize::new(0),
            is_dirty: AtomicBool::new(false),
        }))
    }

    /** @return the pin count of this page */
    #[inline]
    pub fn get_pin_count(&self) -> usize {
        self.0.pin_count.load(Ordering::SeqCst) as usize
    }

    /// Pin page and return the current number of pins
    ///
    /// # Safety
    /// Calling pin multiple times can increase the pin counter more than needed
    ///
    /// only pin once per thread
    pub fn pin(&self) {
        self.0.pin_count.fetch_add(1, Ordering::Relaxed);
        // TODO - add assertion that pin count is not more than ref count
    }

    /// Unpin page and return the current number of pins
    ///
    /// # Safety
    /// Calling unpin multiple times can decrease the pin counter more than needed
    ///
    /// only unpin after each pin
    pub fn unpin(&self) {
        let prev_pin_count = self.0.pin_count.fetch_sub(1, Ordering::Relaxed);

        assert!(prev_pin_count > 0, "Pin count can never be below 0, and pin count is currently {}", prev_pin_count - 1);
    }

    pub fn is_pinned(&self) -> bool {
        self.get_pin_count() > 0
    }

    pub fn is_dirty(&self) -> bool {
        self.0.is_dirty.load(Ordering::SeqCst)
    }

    pub fn set_is_dirty(&self, is_dirty: bool) {
        self.0.is_dirty.store(is_dirty, Ordering::SeqCst)
    }

    pub fn read(&self) -> PageReadGuard {
        self.0.inner.read()
    }

    /// Run function with write lock and get the underlying page
    ///
    /// # Arguments
    ///
    /// * `with_write_lock`: function to run with write lock that get the underlying page
    ///
    /// returns: R `with_write_lock` return value
    ///
    #[inline(always)]
    pub fn with_write<F: FnOnce(&mut UnderlyingPage) -> R, R>(&self, with_write_lock: F) -> R {
        let mut inner_guard = self.0.inner.write();

        with_write_lock(inner_guard.deref_mut())
    }

    pub fn write(&self) -> PageWriteGuard {
        self.0.inner.write()
    }

    pub fn try_write_for(&self, duration: Duration) -> Option<PageWriteGuard> {
        self.0.inner.try_write_for(duration)
    }

    /// Check if the current page is locked in any way
    pub fn is_locked(&self) -> bool {
        self.0.inner.is_locked()
    }

    /// Check if the current page is locked in read
    pub fn is_locked_shared(&self) -> bool {
        self.0.inner.is_locked() && !self.is_locked_exclusive()
    }

    /// Check if the current page is locked in write
    pub fn is_locked_exclusive(&self) -> bool {
        self.0.inner.is_locked_exclusive()
    }


}

impl Default for Page {
    fn default() -> Self {
        Page::new(INVALID_PAGE_ID)
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        let self_guard = self.0.inner.read();
        let other_guard = other.0.inner.read();

        let self_ref = self_guard.deref();
        let other_ref = other_guard.deref();

        self_ref.eq(&other_ref)
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    // TODO - unignore
    #[test]
    #[ignore]
    #[should_panic(expected = "Pin count can never be below 0, and pin count is currently -1")]
    fn should_panic_when_trying_to_unpin_more_than_there_are_pins() {
        let page = Page::new(INVALID_PAGE_ID);

        page.pin();
        page.unpin();
        page.unpin();
    }
}
