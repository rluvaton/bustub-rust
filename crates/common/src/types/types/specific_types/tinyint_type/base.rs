use crate::types::{BUSTUB_I8_MAX, BUSTUB_I8_MIN, BUSTUB_I8_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type TinyIntUnderlyingType = i8;

#[derive(Copy, Debug)]
pub struct TinyIntType {
    pub(in super::super) value: TinyIntUnderlyingType,
    pub(super) len: u32,
}

impl TinyIntType {
    pub const MIN: TinyIntUnderlyingType = BUSTUB_I8_MIN;
    pub const MAX: TinyIntUnderlyingType = BUSTUB_I8_MAX;

    pub fn new(value: TinyIntUnderlyingType) -> Self {
        TinyIntType {
            value,
            len: if value == BUSTUB_I8_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for TinyIntType {
    type Target = TinyIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for TinyIntType {
    fn default() -> Self {
        TinyIntType::new(BUSTUB_I8_NULL)
    }
}
