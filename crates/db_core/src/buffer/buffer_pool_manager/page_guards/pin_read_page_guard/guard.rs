use common::config::{PageData, PageId};
use std::ops::Deref;
use std::sync::Arc;

use crate::buffer::{BufferPoolManager, PinPageGuard, PinWritePageGuard};
use crate::storage::{Page, PageAndWriteGuard, PageUpgradableReadGuard, PageWriteGuard, UnderlyingPage};

#[clippy::has_significant_drop]
#[must_use = "if unused the PinReadPageGuard will immediately unpin and unlock"]
pub struct PinReadPageGuard<'a> {
    // First drop this
    pub(in super::super) read_guard: PageUpgradableReadGuard<'a>,

    // Then drop this
    pub(in super::super) guard: PinPageGuard,
}

impl<'a> PinReadPageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from(PinPageGuard::new(bpm, page.clone()))
    }

    pub fn from_write_guard(bpm: Arc<BufferPoolManager>, page_and_write_guard: PageAndWriteGuard<'a>) -> Self {
        PinWritePageGuard::<'a>::from_write_guard(bpm, page_and_write_guard).into()
    }

    pub fn get_page_id(&self) -> PageId {
        self.read_guard.get_page_id()
    }

    pub fn get_data(&self) -> &PageData {
        self.read_guard.get_data()
    }

    pub fn cast<T>(&self) -> &T {
        self.read_guard.cast::<T>()
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
        self.read_guard.deref()
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
// impl Drop for PinReadPageGuard<'_> {
//     fn drop(&mut self) {
//         unsafe { self.guard.page.unlock_read_without_guard() }
//     }
// }

impl From<PinPageGuard> for PinReadPageGuard<'_> {
    fn from(guard: PinPageGuard) -> Self {
        let read_guard = unsafe { std::mem::transmute::<PageUpgradableReadGuard<'_>, PageUpgradableReadGuard<'static>>(guard.upgradable_read()) };

        PinReadPageGuard {
            read_guard,
            guard,
        }
    }
}

impl<'a> From<PinWritePageGuard<'a>> for PinReadPageGuard<'a> {
    fn from(guard: PinWritePageGuard) -> Self {
        let read_guard = unsafe { std::mem::transmute::<PageUpgradableReadGuard<'_>, PageUpgradableReadGuard<'static>>(PageWriteGuard::downgrade_to_upgradable(guard.write_guard)) };

        PinReadPageGuard {
            read_guard,
            guard: guard.guard,
        }
    }
}
