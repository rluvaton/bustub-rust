use std::ops::Deref;
use std::sync::Arc;
use super::AsPtr;

///
/// # Safety
///
/// The user must make sure:
/// 1. The data will not be `drop`ped or else there will be dangling pointers!
/// 2. The data will not be mutated by other reference or this will lead to undefined behavior!
pub struct UnsafeSingleRefMutData<Ref: AsPtr> {
    r: Arc<*mut Ref::Data>,
}

unsafe impl<Ref: AsPtr> Send for UnsafeSingleRefMutData<Ref> {}
unsafe impl<Ref: AsPtr> Sync for UnsafeSingleRefMutData<Ref> {}

impl<Ref: AsPtr> UnsafeSingleRefMutData<Ref> {
    pub unsafe fn new(r: &mut Ref) -> UnsafeSingleRefMutData<Ref> {
        UnsafeSingleRefMutData {
            r: Arc::new(r.get_mut_ptr())
        }
    }

    pub unsafe fn get_mut<'a>(&self) -> &'a mut Ref::Data {
        Ref::from_mut_ptr(*self.r.deref())
    }
}


impl<Ref: AsPtr> Clone for UnsafeSingleRefMutData<Ref> {
    fn clone(&self) -> Self {
        UnsafeSingleRefMutData {
            r: self.r.clone()
        }
    }
}
