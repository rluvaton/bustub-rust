pub type Underlying = anyhow::Error;
pub type Error = crate::Error<Underlying>;
pub type Result<T> = std::result::Result<T, Error>;

mod macros;

pub use macros::*;