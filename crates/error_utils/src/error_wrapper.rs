use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

pub(crate) trait UnderlyingError: Display + Debug + Send + Sync + 'static {}
impl <E: Display + Debug + Send + Sync + 'static> UnderlyingError for E {}

pub struct ErrorWrapper<E: UnderlyingError> {
    pub(crate) error: anyhow::Error,
    pub(crate) phantom_data: PhantomData<E>
}

unsafe impl<E: UnderlyingError> Send for ErrorWrapper<E> {
}

unsafe impl<E: UnderlyingError> Sync for ErrorWrapper<E> {
}

impl<E: UnderlyingError> Debug for ErrorWrapper<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}


impl<E: UnderlyingError> Deref for ErrorWrapper<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        // Safety: The type system should ensure that the underlying type is the same
        self.error.downcast_ref::<E>().unwrap()
    }
}

impl<E: UnderlyingError + std::error::Error> From<E> for ErrorWrapper<E> {
    fn from(error: E) -> Self {
        ErrorWrapper {
            error: error.into(),
            phantom_data: PhantomData
        }
    }
}
