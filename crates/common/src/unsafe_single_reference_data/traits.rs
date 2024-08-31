pub trait AsPtr {
    type Data: ?Sized;

    unsafe fn get_ptr(&self) -> *const <Self as AsPtr>::Data;

    unsafe fn get_mut_ptr(&mut self) -> *mut <Self as AsPtr>::Data;

    unsafe fn from_ptr<'a>(ptr: *const <Self as AsPtr>::Data) -> &'a <Self as AsPtr>::Data;

    unsafe fn from_mut_ptr<'a>(ptr: *mut <Self as AsPtr>::Data) -> &'a mut <Self as AsPtr>::Data;
}

impl<T> AsPtr for [T] {
    type Data = [T];

    unsafe fn get_ptr(&self) -> *const Self {
        self
    }

    unsafe fn get_mut_ptr(&mut self) -> *mut Self {
        self
    }

    unsafe fn from_ptr<'a>(ptr: *const Self) -> &'a Self {
        &*ptr
    }

    unsafe fn from_mut_ptr<'a>(ptr: *mut Self) -> &'a mut Self {
        &mut *ptr
    }
}

impl<const S: usize, T> AsPtr for [T; S] {
    type Data = [T; S];

    unsafe fn get_ptr(&self) -> *const Self {
        self
    }

    unsafe fn get_mut_ptr(&mut self) -> *mut Self {
        self
    }

    unsafe fn from_ptr<'a>(ptr: *const Self) -> &'a Self {
        &*ptr
    }

    unsafe fn from_mut_ptr<'a>(ptr: *mut Self) -> &'a mut Self {
        &mut *ptr
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_allow_mutate_from_ref() {
        let mut arr = [1, 2, 3];

        let ptr = unsafe { arr.get_mut_ptr() };

        let from_ptr = unsafe { <[i32] as AsPtr>::from_mut_ptr(ptr) };

        from_ptr[0] = 4;

        assert_eq!(arr, [4, 2, 3]);
        assert_eq!(from_ptr, [4, 2, 3]);

        assert_eq!(arr, from_ptr);
    }

    #[test]
    fn should_get_same_data_from_ref() {
        let arr = [1, 2, 3];

        let ptr = unsafe { arr.get_ptr() };

        let from_ptr = unsafe { <[i32] as AsPtr>::from_ptr(ptr) };

        assert_eq!(arr, from_ptr);
    }
}
