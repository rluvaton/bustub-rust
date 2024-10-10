use std::ops::Deref;
use std::sync::Arc;
use pages::{PageData, PageReadGuard, UnderlyingPage};
use super::AsPtr;

///
/// # Safety
///
/// The user must make sure that the data will not be `drop`ped or else there will be dangling pointers!
pub struct PageDataTransfer {
    r: Arc<*const PageData>
}

unsafe impl Send for PageDataTransfer {}
unsafe impl Sync for PageDataTransfer {}

impl PageDataTransfer {
    pub unsafe fn new(r: &PageData) -> Self {
            PageDataTransfer {
                r: Arc::new(r)
            }
    }

    pub unsafe fn get<'a>(&self) -> &'a PageData {
        &*(*self.r)
    }
}

impl From<PageReadGuard<'_>> for PageDataTransfer {
    fn from(value: PageReadGuard) -> Self {
        unsafe { PageDataTransfer::new(value.get_data()) }
    }
}


// impl  Clone for PageDataTransfer {
//     fn clone(&self) -> Self {
//         PageDataTransfer {
//             r: self.r.clone()
//         }
//     }
// }
