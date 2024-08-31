use std::ops::Deref;
use std::sync::Arc;
use super::AsPtr;

///
/// # Safety
///
/// The user must make sure that the data will not be `drop`ped or else there will be dangling pointers!
pub struct UnsafeSingleReferenceReadData<Ref: AsPtr> {
    r: Arc<*const Ref::Data>
}

unsafe impl<Ref: AsPtr> Send for UnsafeSingleReferenceReadData<Ref> {}
unsafe impl<Ref: AsPtr> Sync for UnsafeSingleReferenceReadData<Ref> {}

impl <Ref: AsPtr> UnsafeSingleReferenceReadData<Ref> {
    pub unsafe fn new(r: &Ref) -> UnsafeSingleReferenceReadData<Ref> {
            UnsafeSingleReferenceReadData {
                r: Arc::new(r.get_ptr())
            }
    }

    pub unsafe fn get<'a>(&self) -> &'a Ref::Data {
         Ref::from_ptr(self.r.deref().clone())
    }
}


impl <Ref: AsPtr> Clone for UnsafeSingleReferenceReadData<Ref> {
    fn clone(&self) -> Self {
        UnsafeSingleReferenceReadData {
            r: self.r.clone()
        }
    }
}
