mod base;
mod arithmetics;
mod cmp;
mod tests;
mod conversions;
mod format;
mod storage;
mod constants;

pub use base::{TinyIntType, TinyIntUnderlyingType};

impl crate::DBTypeIdTrait for TinyIntType {

}
