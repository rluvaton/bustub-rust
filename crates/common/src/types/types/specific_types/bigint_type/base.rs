use std::ops::Deref;
use crate::types::{BUSTUB_I64_NULL, BUSTUB_VALUE_NULL};

#[derive(Copy, Debug)]
pub struct BigIntType {
    pub(super) value: i64,
    pub(super) len: u32,
}

impl BigIntType {
    pub fn new(value: i64) -> Self {
        BigIntType {
            value,
            len: if value == BUSTUB_I64_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for BigIntType {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
