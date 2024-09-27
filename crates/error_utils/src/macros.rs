
/// Construct an ad-hoc error from a string or existing non-`anyhow` error
/// value.
///
/// This evaluates to an [`Error`][crate::Error]. It can take either just a
/// string, or a format string with arguments. It also can take any custom type
/// which implements `Debug` and `Display`.
///
/// If called with a single argument whose type implements `std::error::Error`
/// (in addition to `Debug` and `Display`, which are always required), then that
/// Error impl's `source` is preserved as the `source` of the resulting
/// `anyhow::Error`.
///
/// # Example
///
/// ```
/// # type V = ();
/// #
/// use anyhow::{anyhow, Result};
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
        $crate::Error::<$crate::anyhow::Underlying> { error: anyhow::anyhow!($msg), phantom_data: core::marker::PhantomData }
    };
    ($err:expr $(,)?) => {
        $crate::Error::<$crate::anyhow::Underlying> { error: anyhow::anyhow!($err), phantom_data: core::marker::PhantomData }
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::<$crate::anyhow::Underlying> { error: anyhow::anyhow!($fmt, $($arg)*), phantom_data: core::marker::PhantomData }
    };
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn anyhow_macro() {
        let error: crate::anyhow::Error = anyhow!("something");

        // Making sure not crashing
        let value = error.deref();
    }
}
