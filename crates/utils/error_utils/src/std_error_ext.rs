use std::fmt::{Display};
use std::marker::PhantomData;
use crate::error::{Error, UnderlyingError};

pub trait StdErrorExt<E: UnderlyingError> {
    fn ext_context<C>(self, context: C) -> Error<E>
    where
        C: Display + Send + Sync + 'static;
}

impl<E> StdErrorExt<E> for E
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ext_context<C>(self, context: C) -> Error<E>
    where
        C: Display + Send + Sync + 'static,
    {
        let error: Error<E> = Error {
            error: self.into(),
            phantom_data: PhantomData
        };

        error.ext_context(context)
    }
}

impl<E: UnderlyingError> StdErrorExt<E> for Error<E> {
    fn ext_context<C>(self, context: C) -> Error<E>
    where
        C: Display + Send + Sync + 'static,
    {
        Error {
            error: self.error.context(context),
            phantom_data: PhantomData
        }
    }
}

