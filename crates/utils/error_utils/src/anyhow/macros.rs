
/// Mimic [`anyhow!`](anyhow::anyhow) but create [`Error`](crate::Error) instead
///
///
/// # Example
///
/// ```
/// # type V = ();
/// #
/// use error_utils::anyhow::{Result, anyhow};
///
/// fn lookup(key: &str) -> Result<V> {
///     if key.len() != 16 {
///         return Err(anyhow!("key length must be 16 characters, got {:?}", key));
///     }
///
///     // ...
///     # Ok(())
/// }
/// ```
#[macro_export]
macro_rules! anyhow {
    ($msg:literal $(,)?) => {
        $crate::Error::new_anyhow(anyhow::anyhow!($msg))
    };
    ($err:expr $(,)?) => {
        $crate::Error::new_anyhow(anyhow::anyhow!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::new_anyhow(anyhow::anyhow!($fmt, $($arg)*))
    };
}

pub use anyhow;

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn anyhow_macro() {
        let error: crate::anyhow::Error = anyhow!("something");

        // Making sure not crashing
        let _value = error.deref();
    }
}
