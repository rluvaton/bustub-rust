mod base;
mod arithmetics;
mod cmp;
mod tests;
mod conversions;
mod format;
mod storage;
mod constants;

pub use base::{VarcharType, VarcharUnderlyingType};

impl crate::DBTypeIdTrait for VarcharType {

}
