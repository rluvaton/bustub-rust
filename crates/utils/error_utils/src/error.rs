use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

pub trait UnderlyingError: Display + Debug + Send + Sync + 'static {}

impl<E: Display + Debug + Send + Sync + 'static> UnderlyingError for E {}

/// Error wrapper with context support and strong type safety
///
/// # Examples
///
/// Anyhow and std::Error type
///
/// ```
/// use error_utils::ToAnyhowResult;
///
/// // This is how you did it with anyhow
/// fn create_error_using_anyhow(io_error: bool) -> anyhow::Result<()> {
///     if !io_error {
///         return Err(anyhow::anyhow!("hello"));
///     }
///
///     std::fs::OpenOptions::new().open("foo.txt")?;
///
///     Ok(())
/// }
///
/// // And this is how you do it with this crate, note that the type of the error does not save the underlying type here
/// fn create_error_using_error_utils(io_error: bool) -> error_utils::anyhow::Result<()> {
///     if !io_error {
///         return Err(error_utils::anyhow::anyhow!("hello"));
///     }
///
///     std::fs::OpenOptions::new().open("foo.txt").to_anyhow()?;
///
///     Ok(())
/// }
/// ```
///
/// `thiserror::Error` and this crate:
/// ```
///
/// // You need to derive from `error_utils::Error` or implement `CustomError`
/// #[derive(thiserror::Error, Debug)]
/// pub enum MyCustomError {
///     #[error("something 1")]
///     Unknown1,
///     #[error("something 2")]
///     Unknown2
/// }
///
/// fn create_custom_error() -> Result<(), MyCustomError> {
///     Err(MyCustomError::Unknown1)
/// }
///
/// // This is how you did it with `anyhow`, as you can see the error type is not saved
/// fn create_error_using_anyhow(custom_error: bool) -> anyhow::Result<()> {
///     if !custom_error {
///         return Err(MyCustomError::Unknown2.into());
///     }
///
///     create_custom_error()?;
///
///     Ok(())
/// }
///
/// // This is how you do it in with this crate
/// fn create_error_using_error_utils(custom_error: bool) -> Result<(), error_utils::Error<MyCustomError>> {
///     if !custom_error {
///         return Err(MyCustomError::Unknown2.into());
///     }
///
///     create_custom_error()?;
///
///     Ok(())
/// }
///
/// ```
///
pub struct Error<E: UnderlyingError> {
    pub(crate) error: anyhow::Error,
    pub(crate) phantom_data: PhantomData<E>,
}

unsafe impl<E: UnderlyingError> Send for Error<E> {}
unsafe impl<E: UnderlyingError> Sync for Error<E> {}

impl<E:UnderlyingError + PartialEq> PartialEq for Error<E> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}
impl<E:UnderlyingError + Clone + std::error::Error> Clone for Error<E> {
    fn clone(&self) -> Self {
        Error {
            error: self.deref().clone().into(),
            phantom_data: PhantomData
        }
    }
}

impl Error<anyhow::Error> {
    pub fn new_anyhow(error: anyhow::Error) -> Self {
        Self {
            error,
            phantom_data: PhantomData,
        }
    }
}


impl<E: UnderlyingError> Debug for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl<E: UnderlyingError> Display for Error<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
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

// Convert between unknown to StdError
// impl<E> From<E> for Error<crate::anyhow::Underlying>
// where
//     E: StdError + Send + Sync + 'static,
// {
//     #[cold]
//     fn from(error: E) -> Self {
//         Self {
//             error: anyhow::Error::from(error),
//             phantom_data: PhantomData,
//         }
//     }
// }

// impl From<anyhow::Error> for Error<anyhow::Error>
// {
//     #[cold]
//     fn from(error: anyhow::Error) -> Self {
//         Self {
//             error,
//             phantom_data: PhantomData,
//         }
//     }
// }

impl<E> From<E> for Error<E>
where
    E: StdError + Send + Sync + 'static,
{
    #[cold]
    fn from(error: E) -> Self {
        Self {
            error: anyhow::Error::new(error),
            phantom_data: PhantomData,
        }
    }
}


// impl From<anyhow::Error> for Error<anyhow::Error>
// {
//     #[cold]
//     fn from(error: anyhow::Error) -> Self {
//         Self {
//             error,
//             phantom_data: PhantomData,
//         }
//     }
// }

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
    use std::io;
    use crate::ToAnyhowResult;

    #[test]
    fn casting_as_before_for_io_error_and_anyhow() {
        // Making sure that the same type works
        #[allow(dead_code)]
        fn create_error_using_anyhow(io_error: bool) -> anyhow::Result<()> {
            if !io_error {
                return Err(anyhow::anyhow!("hello"));
            }

            std::fs::OpenOptions::new().open("foo.txt")?;

            Ok(())
        }
        
        #[allow(dead_code)]
        fn create_error_using_error_utils(io_error: bool) -> crate::anyhow::Result<()> {
            if !io_error {
                return Err(crate::anyhow::anyhow!("hello"));
            }

            std::fs::OpenOptions::new().open("foo.txt").to_anyhow()?;

            Ok(())
        }
    }

    #[test]
    fn casting_as_before_for_io_error_only() {
        // Making sure that the same type works
        #[allow(dead_code)]
        fn create_error_using_anyhow() -> anyhow::Result<()> {
            std::fs::OpenOptions::new().open("foo.txt")?;

            Ok(())
        }

        #[allow(dead_code)]
        fn create_error_using_error_utils() -> Result<(), crate::Error<io::Error>> {
            std::fs::OpenOptions::new().open("foo.txt")?;

            Ok(())
        }
    }

    #[test]
    fn casting_as_before_for_this_error() {
        #[allow(unused)]
        #[derive(thiserror::Error, Debug)]
        pub enum MyCustomError {
            #[error("something 1")]
            Unknown1,
            #[error("something 2")]
            Unknown2,
        }

        #[allow(dead_code)]
        fn create_custom_error() -> Result<(), MyCustomError> {
            Err(MyCustomError::Unknown1)
        }

        // Making sure that the same type works
        #[allow(dead_code)]
        fn create_error_using_anyhow(custom_error: bool) -> anyhow::Result<()> {
            if !custom_error {
                return Err(MyCustomError::Unknown2.into());
            }

            create_custom_error()?;

            Ok(())
        }

        #[allow(dead_code)]
        fn create_error_using_error_utils(custom_error: bool) -> Result<(), crate::Error<MyCustomError>> {
            if !custom_error {
                return Err(MyCustomError::Unknown2.into());
            }

            create_custom_error()?;

            Ok(())
        }
    }
}
