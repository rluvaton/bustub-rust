use std::fmt::{Debug, Formatter};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use common::config::{PageData, PageId};

use crate::buffer::{AccessType, BufferPoolManager, PinPageGuard, PinReadPageGuard};
use crate::storage::{Page, PageAndReadGuard, PageAndWriteGuard, PageWriteGuard, UnderlyingPage};

#[clippy::has_significant_drop]
#[must_use = "if unused the PinWritePageGuard will immediately unpin and unlock"]
pub struct PinWritePageGuard<'a> {
    // First drop this
    pub(in super::super) write_guard: Option<PageWriteGuard<'a>>,

    pub(in super::super) page: Page,
    pub(in super::super) bpm: Arc<BufferPoolManager>,
}

impl<'a> PinWritePageGuard<'a> {
    pub fn new(bpm: Arc<BufferPoolManager>, page: Page) -> Self {
        Self::from(PinPageGuard::new(bpm, page.clone()))
    }

    pub fn from_write_guard(bpm: Arc<BufferPoolManager>, page_and_write_guard: PageAndWriteGuard<'a>) -> PinWritePageGuard<'a> {
        PinWritePageGuard {
            page: page_and_write_guard.page_ref().clone(),
            write_guard: Some(page_and_write_guard.write_guard()),
            bpm,
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

        let page = mem::take(&mut self.page);

        self.bpm.unpin_page_from_pinned_page(&page, AccessType::default());

        // unsafe { self.guard.page.unlock_write_without_guard(); }
    }
}

impl From<PinPageGuard> for PinWritePageGuard<'_> {
    fn from(guard: PinPageGuard) -> Self {
        let write_guard = unsafe { std::mem::transmute::<PageWriteGuard<'_>, PageWriteGuard<'static>>(guard.write()) };

        let page = guard.page.clone();
        let bpm = guard.bpm.clone();

        // Avoid unpinning
        mem::forget(guard);

        PinWritePageGuard {
            write_guard: Some(write_guard),
            page,
            bpm,
        }
    }
}

// impl<'a> From<PinReadPageGuard<'a>> for PinWritePageGuard<'a> {
//     fn from(mut guard: PinReadPageGuard) -> Self {
//         let new_guard = unsafe {
//             match &guard.guard {
//                 Some(v) => v.create_new(),
//                 _ => unreachable!()
//             }
//         };
//
//         // Release the read lock
//         drop(mem::take(&mut guard.read_guard));
//
//         // Avoid guard being unpinned
//         mem::forget(mem::take(&mut guard.guard));
//
//         PinWritePageGuard::from(new_guard)
//     }
// }

impl Debug for PinWritePageGuard<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.write_guard {
            Some(u) => write!(f, "pin write page guard for page {}", u.get_page_id()),
            None => write!(f, "pin write page guard for unknown page")
        }
    }
}
