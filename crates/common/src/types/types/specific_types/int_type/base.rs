use crate::types::{BUSTUB_I32_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type IntUnderlyingType = i32;

#[derive(Copy, Debug)]
pub struct IntType {
    pub(in super::super) value: IntUnderlyingType,
    pub(super) len: u32,
}

impl IntType {
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
