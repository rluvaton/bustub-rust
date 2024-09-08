use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use common::config::{PageData, PageId};

use crate::buffer::{BufferPoolManager, PinPageGuard};
use crate::storage::{Page, UnderlyingPage};

#[clippy::has_significant_drop]
#[must_use = "if unused the PinWritePageGuard will immediately unpin and unlock"]
pub struct PinWritePageGuard<'a> {
    pub(crate) underlying_page: &'a mut UnderlyingPage,
    pub(crate) guard: PinPageGuard,
}

impl<'a> PinWritePageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from_guard(PinPageGuard::new(bpm, page.clone()), page)
    }

    pub fn from_guard(guard: PinPageGuard, page: Page) -> Self {
        unsafe {
            PinWritePageGuard {
                underlying_page: &mut *page.write_without_guard(),
                guard,
            }
        }
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
