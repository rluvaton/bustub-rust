use crate::types::{TinyIntUnderlyingType, BUSTUB_I32_MAX, BUSTUB_I32_MIN, BUSTUB_I32_NULL, BUSTUB_I8_MAX, BUSTUB_I8_MIN, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type IntUnderlyingType = i32;

#[derive(Copy, Debug)]
pub struct IntType {
    pub(in super::super) value: IntUnderlyingType,
    pub(super) len: u32,
}

impl IntType {
    pub const MIN: IntUnderlyingType = BUSTUB_I32_MIN;
    pub const MAX: IntUnderlyingType = BUSTUB_I32_MAX;

    pub fn new(value: IntUnderlyingType) -> Self {
        IntType {
            value,
            len: if value == BUSTUB_I32_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for IntType {
    type Target = IntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for IntType {
    fn default() -> Self {
        IntType::new(BUSTUB_I32_NULL)
    }
}
