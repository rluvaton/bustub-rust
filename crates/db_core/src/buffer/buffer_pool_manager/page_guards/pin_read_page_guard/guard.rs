use std::fmt::Debug;
use std::mem;
use common::config::{PageData, PageId};
use std::ops::Deref;
use std::sync::Arc;

use crate::buffer::{AccessType, BufferPoolManager, PinPageGuard, PinWritePageGuard};
use crate::storage::{Page, PageAndReadGuard, PageAndWriteGuard, PageReadGuard, PageWriteGuard, UnderlyingPage};

use super::super::PageLockComparator;

#[derive(Debug)]
pub enum UpgradePinWriteLockError<PageLockComparatorImpl: PageLockComparator> {
    // #[error("page id changed (was {0}) when trying to upgrade write lock")]
    PageIdChanged(PageId),

    // #[error("page is not the same {0}")]
    PageLockComparatorError(PageLockComparatorImpl::CompareError),
}


#[clippy::has_significant_drop]
#[must_use = "if unused the PinReadPageGuard will immediately unpin and unlock"]
pub struct PinReadPageGuard<'a> {
    // First drop this
    pub(in super::super) read_guard: Option<PageReadGuard<'a>>,

    pub(in super::super) page: Page,
    pub(in super::super) bpm: Arc<BufferPoolManager>,
}

impl<'a> PinReadPageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from(PinPageGuard::new(bpm, page.clone()))
    }

    pub fn from_read_guard(bpm: Arc<BufferPoolManager>, page_and_read_guard: PageAndReadGuard<'a>) -> Self {
        Self {
            page: page_and_read_guard.page_ref().clone(),
            read_guard: Some(page_and_read_guard.read_guard()),
            bpm,
        }
    }

    pub fn from_write_guard(bpm: Arc<BufferPoolManager>, page_and_write_guard: PageAndWriteGuard<'a>) -> Self {
        PinWritePageGuard::<'a>::from_write_guard(bpm, page_and_write_guard).into()
    }

    pub fn get_page_id(&self) -> PageId {
        match &self.read_guard {
            Some(u) => u.get_page_id(),
            None => unreachable!()
        }
    }

    pub fn get_data(&self) -> &PageData {
        match &self.read_guard {
            Some(u) => u.get_data(),
            None => unreachable!()
        }
    }

    pub fn cast<T>(&self) -> &T {
        match &self.read_guard {
            Some(u) => u.cast::<T>(),
            None => unreachable!()
        }
    }

    /// * TODO(P2): Add implementation
    ///
    ///  @brief Move assignment for ReadPageGuard
    ///
    ///  Very similar to BasicPageGuard. Given another ReadPageGuard,
    ///  replace the contents of this one with that one.
    ///
    pub fn replace_inner(&mut self) {
        unimplemented!()
    }

    pub fn try_upgrade_write<PageLockComparatorImpl: PageLockComparator>(mut self) -> Result<PinWritePageGuard<'a>, UpgradePinWriteLockError<PageLockComparatorImpl>> {
        let read_guard = mem::take(&mut self.read_guard).unwrap();
        let comparator = PageLockComparatorImpl::new(read_guard.deref());

        let page_id = read_guard.get_page_id();

        // Release the read lock
        drop(read_guard);

        let page = self.page.clone();
        let bpm = self.bpm.clone();

        // Avoid unpinning
        mem::forget(self);

        let write_guard = PinWritePageGuard::new(bpm, page);

        // Avoid guard being unpinned, this should be before the compare
        // as if the compare panic we won't unpin twice (one in the new_guard and on from the self.guard

        comparator.compare(write_guard.deref()).map_err(|err| UpgradePinWriteLockError::PageLockComparatorError(err))?;

        if write_guard.get_page_id() != page_id {
            return Err(UpgradePinWriteLockError::PageIdChanged(page_id));
        }

        Ok(write_guard)
    }
}

impl Deref for PinReadPageGuard<'_> {
    type Target = UnderlyingPage;

    #[inline]
    fn deref(&self) -> &UnderlyingPage {
        match &self.read_guard {
            Some(v) => v.deref(),
            _ => unreachable!()
        }
    }
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
impl Drop for PinReadPageGuard<'_> {
    fn drop(&mut self) {
        let read_guard = mem::take(&mut self.read_guard);

        // Drop read lock
        drop(read_guard);

        // Acquire write lock, it should still be
        self.bpm.unpin_page_from_pinned_page(&mut self.page, AccessType::default());
    }
}

impl From<PinPageGuard> for PinReadPageGuard<'_> {
    fn from(guard: PinPageGuard) -> Self {
        let read_guard = unsafe { std::mem::transmute::<PageReadGuard<'_>, PageReadGuard<'static>>(guard.read()) };

        let page = guard.page.clone();
        let bpm = guard.bpm.clone();

        // Avoid unpinning
        mem::forget(guard);

        PinReadPageGuard {
            read_guard: Some(read_guard),
            page,
            bpm,
        }
    }
}

impl<'a> From<PinWritePageGuard<'a>> for PinReadPageGuard<'a> {
    fn from(mut guard: PinWritePageGuard) -> Self {
        let read_guard = unsafe { std::mem::transmute::<PageReadGuard<'_>, PageReadGuard<'static>>(PageWriteGuard::downgrade(mem::take(&mut guard.write_guard).unwrap())) };

        let page = guard.page.clone();
        let bpm = guard.bpm.clone();

        // Avoid unpinning
        mem::forget(guard);

        PinReadPageGuard {
            read_guard: Some(read_guard),
            page,
            bpm,
        }
    }
}
