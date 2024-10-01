use std::mem;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use common::config::{PageData, PageId};

use crate::buffer::{BufferPoolManager, PinPageGuard, PinReadPageGuard};
use crate::storage::{Page, PageAndReadGuard, PageAndWriteGuard, PageWriteGuard, UnderlyingPage};

#[clippy::has_significant_drop]
#[must_use = "if unused the PinWritePageGuard will immediately unpin and unlock"]
pub struct PinWritePageGuard<'a> {
    // First drop this
    pub(in super::super) write_guard: Option<PageWriteGuard<'a>>,

    // Then drop this
    pub(in super::super) guard: Option<PinPageGuard>,
}

impl<'a> PinWritePageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from(PinPageGuard::new(bpm, page.clone()))
    }


    pub fn from_read_guard(bpm: Arc<BufferPoolManager>, page_and_read_guard: PageAndReadGuard<'a>) -> Self {
        Self::from(PinReadPageGuard::from_read_guard(bpm, page_and_read_guard))
    }

    pub fn from_write_guard(bpm: Arc<BufferPoolManager>, page_and_write_guard: PageAndWriteGuard<'a>) -> PinWritePageGuard<'a> {
        let guard = PinPageGuard::new(bpm, page_and_write_guard.page_ref().clone());

        PinWritePageGuard {
            write_guard: Some(page_and_write_guard.write_guard()),
            guard: Some(guard),
        }
    }

    pub fn get_page_id(&self) -> PageId {
        match &self.write_guard {
            Some(u) => u.get_page_id(),
            _ => unreachable!()
        }
    }

    pub fn get_data_mut(&mut self) -> &mut PageData {
        match &mut self.write_guard {
            Some(u) => u.get_data_mut(),
            _ => unreachable!()
        }
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

    pub fn downgrade_to_read(self) -> PinReadPageGuard<'a> {
        PinReadPageGuard::from(self)
    }
}

impl Deref for PinWritePageGuard<'_> {
    type Target = UnderlyingPage;

    #[inline]
    fn deref(&self) -> &UnderlyingPage {
        match &self.write_guard {
            Some(u) => u,
            _ => unreachable!()
        }
    }
}

impl DerefMut for PinWritePageGuard<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.write_guard {
            Some(u) => u,
            _ => unreachable!()
        }
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
impl Drop for PinWritePageGuard<'_> {
    fn drop(&mut self) {
        // unsafe { self.guard.page.unlock_write_without_guard(); }
    }
}

impl From<PinPageGuard> for PinWritePageGuard<'_> {
    fn from(guard: PinPageGuard) -> Self {
        let write_guard = unsafe { std::mem::transmute::<PageWriteGuard<'_>, PageWriteGuard<'static>>(guard.write()) };

        PinWritePageGuard {
            write_guard: Some(write_guard),
            guard: Some(guard),
        }
    }
}

impl<'a> From<PinReadPageGuard<'a>> for PinWritePageGuard<'a> {
    fn from(mut guard: PinReadPageGuard) -> Self {
        let new_guard = unsafe {
            match &guard.guard {
                Some(v) => v.create_new(),
                _ => unreachable!()
            }
        };

        // Release the read lock
        drop(mem::take(&mut guard.read_guard));

        // Avoid guard being unpinned
        mem::forget(mem::take(&mut guard.guard));

        PinWritePageGuard::from(new_guard)
    }
}
