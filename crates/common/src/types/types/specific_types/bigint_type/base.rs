use std::ops::Deref;
use crate::types::{BUSTUB_I64_NULL, BUSTUB_VALUE_NULL};

pub(super) type BigIntUnderlyingType = i64;

#[derive(Copy, Debug)]
pub struct BigIntType {
    pub(in super::super) value: BigIntUnderlyingType,
    pub(super) len: u32,
}

impl BigIntType {
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
