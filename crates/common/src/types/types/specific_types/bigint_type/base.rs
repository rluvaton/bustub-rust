use std::ops::Deref;
use crate::types::{BUSTUB_I64_NULL, BUSTUB_VALUE_NULL};
use crate::types::types::specific_types_trait::UnderlyingDBTypeTrait;

#[derive(Copy, Debug)]
pub struct BigIntType {
    pub(super) value: <Self as UnderlyingDBTypeTrait>::UnderlyingType,
    pub(super) len: u32,
}

impl BigIntType {
    pub fn new(value: <Self as UnderlyingDBTypeTrait>::UnderlyingType) -> Self {
        BigIntType {
            value,
            len: if value == BUSTUB_I64_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for BigIntType {
    type Target = <Self as UnderlyingDBTypeTrait>::UnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UnderlyingDBTypeTrait for BigIntType {
    type UnderlyingType = i64;
}
