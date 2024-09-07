use crate::types::{BUSTUB_I64_MAX, BUSTUB_I64_MIN, BUSTUB_I64_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type BigIntUnderlyingType = i64;

#[derive(Copy, Debug)]
pub struct BigIntType {
    pub(in crate) value: BigIntUnderlyingType,
    pub(super) len: u32,
}

impl BigIntType {
    pub const MIN: BigIntUnderlyingType = BUSTUB_I64_MIN;
    pub const MAX: BigIntUnderlyingType = BUSTUB_I64_MAX;

    pub fn new(value: BigIntUnderlyingType) -> Self {
        BigIntType {
            value,
            len: if value == BUSTUB_I64_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for BigIntType {
    type Target = BigIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for BigIntType {
    fn default() -> Self {
        BigIntType::new(BUSTUB_I64_NULL)
    }
}
