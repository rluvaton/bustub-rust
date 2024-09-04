use crate::buffer_pool_manager::PinPageGuard;
use common::config::{PageData, PageId};
use std::ops::Deref;
use std::sync::Arc;
use storage::storage::{Page, UnderlyingPage};
use crate::BufferPoolManager;

#[clippy::has_significant_drop]
#[must_use = "if unused the PinReadPageGuard will immediately unpin and unlock"]
pub struct PinReadPageGuard<'a> {
    pub(crate) underlying_page: &'a UnderlyingPage,
    pub(crate) guard: PinPageGuard,
}

impl<'a> PinReadPageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from_guard(PinPageGuard::new(bpm, page.clone()), page)
    }

    pub fn from_guard(guard: PinPageGuard, page: Page) -> Self {
        unsafe {
            PinReadPageGuard {
                underlying_page: &*page.read_without_guard(),
                guard,
            }
        }
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
