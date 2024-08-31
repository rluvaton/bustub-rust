use std::ops::Deref;
use std::sync::Arc;
use super::AsPtr;

///
/// # Safety
///
/// The user must make sure:
/// 1. The data will not be `drop`ped or else there will be dangling pointers!
/// 2. The data will not be mutated by other reference or this will lead to undefined behavior!
pub struct UnsafeSingleReferenceWriteData<Ref: AsPtr> {
    r: Arc<*mut Ref::Data>,
}

unsafe impl<Ref: AsPtr> Send for UnsafeSingleReferenceWriteData<Ref> {}
unsafe impl<Ref: AsPtr> Sync for UnsafeSingleReferenceWriteData<Ref> {}

impl<Ref: AsPtr> UnsafeSingleReferenceWriteData<Ref> {
    pub unsafe fn new(r: &mut Ref) -> UnsafeSingleReferenceWriteData<Ref> {
        UnsafeSingleReferenceWriteData {
            r: Arc::new(r.get_mut_ptr())
        }
    }

    pub unsafe fn get_mut<'a>(&self) -> &'a mut Ref::Data {
        Ref::from_mut_ptr(*self.r.deref())
    }
}


impl<Ref: AsPtr> Clone for UnsafeSingleReferenceWriteData<Ref> {
    fn clone(&self) -> Self {
        UnsafeSingleReferenceWriteData {
            r: self.r.clone()
        }
    }
}
