use std::ops::Deref;
use std::sync::Arc;
use pages::{PageData, PageWriteGuard, UnderlyingPage};
use super::AsPtr;

///
/// # Safety
///
/// The user must make sure:
/// 1. The data will not be `drop`ped or else there will be dangling pointers!
/// 2. The data will not be mutated by other reference or this will lead to undefined behavior!
///
///
/// # Safety
///
/// The user must make sure that the data will not be `drop`ped or else there will be dangling pointers!
pub struct MutPageDataTransfer {
    r: Arc<*mut PageData>
}

unsafe impl Send for MutPageDataTransfer {}
unsafe impl Sync for MutPageDataTransfer {}

impl MutPageDataTransfer {
    pub unsafe fn new(r: &mut PageData) -> Self {
        MutPageDataTransfer {
            r: Arc::new(r)
        }
    }

    pub unsafe fn get_mut<'a>(&mut self) -> &'a mut PageData {
        &mut **self.r
    }
}

impl From<PageWriteGuard<'_>> for MutPageDataTransfer {
    fn from(mut value: PageWriteGuard<'_>) -> Self {
        unsafe { MutPageDataTransfer::new(value.get_data_mut()) }
    }
}
// pub struct UnsafeSingleRefMutData<Ref: AsPtr> {
//     r: Arc<*mut Ref::Data>,
// }
//
// unsafe impl<Ref: AsPtr> Send for UnsafeSingleRefMutData<Ref> {}
// unsafe impl<Ref: AsPtr> Sync for UnsafeSingleRefMutData<Ref> {}
//
// impl<Ref: AsPtr> UnsafeSingleRefMutData<Ref> {
//     pub unsafe fn new(r: &mut Ref) -> UnsafeSingleRefMutData<Ref> {
//         UnsafeSingleRefMutData {
//             r: Arc::new(r.get_mut_ptr())
//         }
//     }
//
//     pub unsafe fn get_mut<'a>(&self) -> &'a mut Ref::Data {
//         Ref::from_mut_ptr(*self.r.deref())
//     }
// }
//
//
// impl<Ref: AsPtr> Clone for UnsafeSingleRefMutData<Ref> {
//     fn clone(&self) -> Self {
//         UnsafeSingleRefMutData {
//             r: self.r.clone()
//         }
//     }
// }
