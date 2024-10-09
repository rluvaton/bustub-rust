pub type Underlying = anyhow::Error;
pub type Error = crate::Error<anyhow::Error>;
pub type Result<T> = std::result::Result<T, Error>;

mod macros;
mod traits;

pub use macros::*;
pub use traits::{ToAnyhow, ToAnyhowResult};