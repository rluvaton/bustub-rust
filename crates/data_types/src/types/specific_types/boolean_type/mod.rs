mod base;
mod arithmetics;
mod cmp;
mod tests;
mod conversions;
mod format;
mod storage;
mod constants;

pub use base::{BooleanType, BooleanUnderlyingType};

impl crate::DBTypeIdTrait for BooleanType {

}
