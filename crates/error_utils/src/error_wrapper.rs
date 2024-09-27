use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::Deref;

pub(crate) trait UnderlyingError: Display + Debug + Send + Sync + 'static {}
impl <E: Display + Debug + Send + Sync + 'static> UnderlyingError for E {}

pub struct Error<E: UnderlyingError> {
    pub(crate) error: anyhow::Error,
    pub(crate) phantom_data: PhantomData<E>
}

unsafe impl<E: UnderlyingError> Send for Error<E> {}
unsafe impl<E: UnderlyingError> Sync for Error<E> {}

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
//
// impl Deref for Error<UnderlyingAnyhowError> {
//     type Target = UnderlyingAnyhowError;
//
//     fn deref(&self) -> &Self::Target {
//         &self.error
//     }
// }

impl<E: UnderlyingError + std::error::Error> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error {
            error: error.into(),
            phantom_data: PhantomData
        }
    }
}

// impl From<UnderlyingAnyhowError> for Error<UnderlyingAnyhowError> {
//     fn from(error: UnderlyingAnyhowError) -> Self {
//         Error {
//             error: error,
//             phantom_data: PhantomData
//         }
//     }
// }
