use std::mem;
use std::sync::Arc;
use std::ops::Deref;

use common::config::{PageData, PageId};

use crate::buffer::{BufferPoolManager, PinPageGuard, PinWritePageGuard};
use crate::storage::{Page, UnderlyingPage};

#[clippy::has_significant_drop]
#[must_use = "if unused the PinReadPageGuard will immediately unpin and unlock"]
pub struct PinReadPageGuard<'a> {
    pub(in super::super) underlying_page: &'a UnderlyingPage,
    pub(in super::super) guard: PinPageGuard,
}

impl<'a> PinReadPageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from(PinPageGuard::new(bpm, page.clone()))
    }

    pub fn get_page_id(&self) -> PageId {
        self.underlying_page.get_page_id()
    }

    pub fn get_data(&self) -> &PageData {
        self.underlying_page.get_data()
    }

    pub fn cast<T>(&self) -> &T {
        self.underlying_page.cast::<T>()
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

    pub fn upgrade_write(self) -> PinWritePageGuard<'a> {
        PinWritePageGuard::from(self)
    }
}

impl Deref for PinReadPageGuard<'_> {
    type Target = UnderlyingPage;

    #[inline]
    fn deref(&self) -> &UnderlyingPage {
        &self.underlying_page
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
        unsafe { self.guard.page.unlock_read_without_guard() }
    }
}

impl From<PinPageGuard> for PinReadPageGuard<'_> {
    fn from(guard: PinPageGuard) -> Self {
        let page = guard.page.clone();

        unsafe {
            PinReadPageGuard {
                underlying_page: &*page.read_without_guard(),
                guard,
            }
        }
    }
}

impl<'a> From<PinWritePageGuard<'a>> for PinReadPageGuard<'a> {
    fn from(guard: PinWritePageGuard) -> Self {
        let pin_guard = unsafe {
            guard.guard.create_new()
        };

        let underlying_page: &UnderlyingPage;

        unsafe {
            // Unlock write manually before acquiring read as it will lead to a deadlock
            // due to we only release the write lock on PinWritePageGuard drop
            pin_guard.unlock_write_without_guard();

            // Acquire read
            underlying_page = &*pin_guard.read_without_guard()
        };

        // Do not run:
        // 1. Pin guard drop function: as we don't want to unpin, we transfer it
        // 2. This guard drop function: as we already manually unlocked it and unlocking again will lead to an undefined behavior
        mem::forget(guard);

        unsafe {
            PinReadPageGuard {
                underlying_page,
                guard: pin_guard,
            }
        }
    }
}
