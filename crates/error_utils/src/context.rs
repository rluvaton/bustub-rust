use crate::error::{Error};
use crate::std_error_ext::StdErrorExt;
use std::error::Error as StdError;
use std::fmt::{Debug, Display};
use std::ops::Deref;
use crate::UnderlyingError;

pub trait Context<T, E: UnderlyingError> {
    /// Wrap the error value with additional context.
    fn context<C>(self, context: C) -> Result<T, Error<E>>
    where
        C: 'static + Display + Send + Sync;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_context<C, F>(self, f: F) -> Result<T, Error<E>>
    where
        C: 'static + Display + Send + Sync,
        F: FnOnce() -> C;
}


impl<T, E> Context<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, Error<E>>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context)),
        }
    }

    fn with_context<C, F>(self, context: F) -> Result<T, Error<E>>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context())),
        }
    }
}

impl<T, E> Context<T, E> for Result<T, Error<E>>
where
    E: UnderlyingError,
{
    fn context<C>(self, context: C) -> Result<T, Error<E>>
    where
        C: Display + Send + Sync + 'static,
    {
        // Not using map_err to save 2 useless frames off the captured backtrace
        // in ext_context.
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context)),
        }
    }

    fn with_context<C, F>(self, context: F) -> Result<T, Error<E>>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(error) => Err(error.ext_context(context())),
        }
    }
}


#[cfg(test)]
mod tests {
use std::fmt::Display;

    fn remove_stack_trace(error_string: String) -> String {
        error_string.split_at(error_string.find("Stack backtrace:").unwrap()).0.to_string()
    }

    #[test]
    fn context_output() {
        #[derive(thiserror::Error, Debug)]
        enum MyCustomError {
            #[error("This is something {0}")]
            Something(i8)
        }

        type MyCustomErrorWithContext = crate::Error<MyCustomError>;

        fn some_fn() -> Result<(), MyCustomError> {
            Err(MyCustomError::Something(1))
        }

        let own_output: String;
        let anyhow_output: String;

        {
            use crate::Context;

            fn consumer2() -> Result<(), MyCustomErrorWithContext> {
                some_fn().context("hello")?;

                Ok(())
            }

            fn consumer1() -> Result<(), MyCustomErrorWithContext> {
                consumer2().context("2")?;

                Ok(())
            }

            let result = consumer1();
            own_output = format!("{:?}", result.unwrap_err())
        }

        {
            use anyhow::Context;

            fn consumer2() -> Result<(), anyhow::Error> {
                some_fn().context("hello")?;

                Ok(())
            }

            fn consumer1() -> Result<(), anyhow::Error> {
                consumer2().context("2")?;

                Ok(())
            }

            let result = consumer1();

            anyhow_output = format!("{:?}", result.unwrap_err())
        }

        assert_eq!(remove_stack_trace(own_output), remove_stack_trace(anyhow_output), "Own output should equal to anyhow output")
    }
}
