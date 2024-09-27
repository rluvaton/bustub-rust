use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use crate::error_wrapper::{ErrorWrapper, UnderlyingError};

pub trait StdErrorExt<E: UnderlyingError> {
    fn ext_context<C>(self, context: C) -> ErrorWrapper<E>
    where
        C: Display + Send + Sync + 'static;
}

impl<E> StdErrorExt<E> for E
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ext_context<C>(self, context: C) -> ErrorWrapper<E>
    where
        C: Display + Send + Sync + 'static,
    {
        let error: ErrorWrapper<E> = self.into();

        error.ext_context(context)
    }
}

impl<E: UnderlyingError> StdErrorExt<E> for ErrorWrapper<E> {
    fn ext_context<C>(self, context: C) -> ErrorWrapper<E>
    where
        C: Display + Send + Sync + 'static,
    {
        ErrorWrapper {
            error: self.error.context(context),
            phantom_data: PhantomData
        }
    }
}

