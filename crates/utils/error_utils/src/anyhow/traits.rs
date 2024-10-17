use std::marker::PhantomData;
use crate::Error;

pub trait ToAnyhow {
    fn to_anyhow(self) -> crate::Error<anyhow::Error>;
}

pub trait ToAnyhowResult<T> {
    fn to_anyhow(self) -> Result<T, crate::Error<anyhow::Error>>;
}

impl<E: Into<anyhow::Error>> ToAnyhow for E {
    fn to_anyhow(self) -> Error<anyhow::Error> {
        Error {
            error: self.into(),
            phantom_data: PhantomData
        }
    }
}

impl<T, E: Into<anyhow::Error>> ToAnyhowResult<T> for Result<T, E> {
    fn to_anyhow(self) -> Result<T, crate::Error<anyhow::Error>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error {
                error: err.into(),
                phantom_data: PhantomData
            })
        }
    }
}


impl<T, E: Into<anyhow::Error> + std::fmt::Display + std::fmt::Debug + std::marker::Send + std::marker::Sync + 'static> ToAnyhowResult<T> for Result<T, Error<E>> {
    fn to_anyhow(self) -> Result<T, crate::Error<anyhow::Error>> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error {
                error: err.error.into(),
                phantom_data: PhantomData
            })
        }
    }
}
