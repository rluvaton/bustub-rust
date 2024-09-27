use std::any::Any;
use std::backtrace::Backtrace;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;
use std::error::Error as StdError;


pub trait UnderlyingError: Display + Debug + Send + Sync + 'static {}
impl <E: Display + Debug + Send + Sync + 'static> UnderlyingError for E {}
pub trait CustomError: Sized {
    fn create_anyhow(self) -> anyhow::Error where Self: UnderlyingError + StdError {
        anyhow::Error::new(self)
    }
}

pub struct Error<E: UnderlyingError> {
    pub(crate) error: anyhow::Error,
    pub(crate) phantom_data: PhantomData<E>
}

unsafe impl<E: UnderlyingError> Send for Error<E> {}
unsafe impl<E: UnderlyingError> Sync for Error<E> {}

impl Error<crate::anyhow::Underlying> {
    pub fn new_anyhow(error: crate::anyhow::Underlying) -> Self {
        Self {
            error,
            phantom_data: PhantomData
        }
    }
}

impl<E: UnderlyingError> Debug for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl<E: UnderlyingError> Deref for Error<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        let value_any = &self.error as &dyn Any;

        // Try to convert our value to a `String`. If successful, we want to
        // output the `String`'s length as well as its value. If not, it's a
        // different type: just print it out unadorned.
        match value_any.downcast_ref::<E>() {
            Some(as_underlying) => {
               as_underlying
            }
            None => {
                // Safety: The type system should ensure that the underlying type is the same
                self.error.downcast_ref::<E>().unwrap()
            }
        }
    }
}

impl<E> From<E> for Error<crate::anyhow::Underlying>
where
    E: StdError + Send + Sync + 'static,
{
    #[cold]
    fn from(error: E) -> Self {
        Self {
            error: anyhow::Error::from(error),
            phantom_data: PhantomData
        }
    }
}

impl<E> From<E> for Error<E>
where
    E: CustomError + Send + Sync + 'static + std::fmt::Display + std::fmt::Debug + std::error::Error,
{
    #[cold]
    fn from(error: E) -> Self {
        Self {
            error: error.create_anyhow(),
            phantom_data: PhantomData
        }
    }
}

// impl<E: UnderlyingError + std::error::Error> From<E> for Error<E> {
//     fn from(error: E) -> Self {
//         Error {
//             error: error.into(),
//             phantom_data: PhantomData
//         }
//     }
// }

//
// impl<E: Into<anyhow::Error>> From<E> for Error<anyhow::Error> {
//     fn from(error: E) -> Self {
//         Error {
//             error: error.into(),
//             phantom_data: PhantomData
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use crate::CustomError;

    #[test]
    fn casting_as_before_for_io_error() {
        // Making sure that the same type works
        fn create_error_using_anyhow(io_error: bool) -> anyhow::Result<()> {
            if !io_error {
                return Err(anyhow::anyhow!("hello"));
            }

            OpenOptions::new().open("foo.txt")?;

            Ok(())
        }

        fn create_error_using_error_utils(io_error: bool) -> crate::anyhow::Result<()> {
            if !io_error {
                return Err(crate::anyhow::anyhow!("hello"));
            }

            OpenOptions::new().open("foo.txt")?;

            Ok(())
        }
    }

    #[test]
    fn casting_as_before_for_this_error() {
        #[derive(thiserror::Error, crate::Error, Debug, PartialEq, Clone)]
        pub enum MyCustomError {
            #[error("something 1")]
            Unknown1,
            #[error("something 2")]
            Unknown2
        }

        fn create_custom_error() -> Result<(), MyCustomError> {
            Err(MyCustomError::Unknown1)
        }

        // Making sure that the same type works
        fn create_error_using_anyhow(custom_error: bool) -> anyhow::Result<()> {
            if !custom_error {
                return Err(anyhow::anyhow!("using anyhow"));
            }

            create_custom_error()?;

            Ok(())
        }

        fn create_error_using_error_utils(custom_error: bool) -> Result<(), crate::Error<MyCustomError>> {
            if !custom_error {
                return Err(MyCustomError::Unknown2.into());
            }

            create_custom_error()?;

            Ok(())
        }
    }
}
