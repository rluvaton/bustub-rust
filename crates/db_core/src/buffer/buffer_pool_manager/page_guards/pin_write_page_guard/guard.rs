use std::mem;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use common::config::{PageData, PageId};

use crate::buffer::{BufferPoolManager, PinPageGuard, PinReadPageGuard};
use crate::storage::{Page, UnderlyingPage};

#[clippy::has_significant_drop]
#[must_use = "if unused the PinWritePageGuard will immediately unpin and unlock"]
pub struct PinWritePageGuard<'a> {
    pub(in super::super) underlying_page: &'a mut UnderlyingPage,
    pub(in super::super) guard: PinPageGuard,
}

impl<'a> PinWritePageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from(PinPageGuard::new(bpm, page.clone()))
    }

    pub fn get_page_id(&self) -> PageId {
        self.underlying_page.get_page_id()
    }

    pub fn get_data_mut(&mut self) -> &mut PageData {
        self.underlying_page.get_data_mut()
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
        &self.underlying_page
    }
}

impl DerefMut for PinWritePageGuard<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.underlying_page
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
        unsafe { self.guard.page.unlock_write_without_guard(); }
    }
}

impl From<PinPageGuard> for PinWritePageGuard<'_> {
    fn from(guard: PinPageGuard) -> Self {
        let page = guard.page.clone();

        unsafe {
            PinWritePageGuard {
                underlying_page: &mut *page.write_without_guard(),
                guard,
            }
        }
    }
}

impl<'a> From<PinReadPageGuard<'a>> for PinWritePageGuard<'a> {
    fn from(guard: PinReadPageGuard) -> Self {
        let pin_guard = unsafe {
            guard.guard.create_new()
        };

        let underlying_page: &mut UnderlyingPage;

        unsafe {
            // Unlock read manually before acquiring write as it will lead to a deadlock
            // due to we only release the read lock on PinReadPageGuard drop
            pin_guard.unlock_read_without_guard();

            // Acquire write
            underlying_page = &mut *pin_guard.write_without_guard()
        };


        // Do not run:
        // 1. Pin guard drop function: as we don't want to unpin, we transfer it
        // 2. This guard drop function: as we already manually unlocked it and unlocking again will lead to an undefined behavior
        mem::forget(guard);

        unsafe {
            PinWritePageGuard {
                underlying_page,
                guard: pin_guard,
            }
        }
    }
}
