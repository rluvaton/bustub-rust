use std::ops::Deref;
use std::sync::Arc;
use super::AsPtr;

///
/// # Safety
///
/// The user must make sure that the data will not be `drop`ped or else there will be dangling pointers!
pub struct UnsafeSingleRefData<Ref: AsPtr> {
    r: Arc<*const Ref::Data>
}

unsafe impl<Ref: AsPtr> Send for UnsafeSingleRefData<Ref> {}
unsafe impl<Ref: AsPtr> Sync for UnsafeSingleRefData<Ref> {}

impl <Ref: AsPtr> UnsafeSingleRefData<Ref> {
    pub unsafe fn new(r: &Ref) -> UnsafeSingleRefData<Ref> {
            UnsafeSingleRefData {
                r: Arc::new(r.get_ptr())
            }
    }

    pub unsafe fn get<'a>(&self) -> &'a Ref::Data {
         Ref::from_ptr(self.r.deref().clone())
    }
}


impl <Ref: AsPtr> Clone for UnsafeSingleRefData<Ref> {
    fn clone(&self) -> Self {
        UnsafeSingleRefData {
            r: self.r.clone()
        }
    }
}
