use crate::types::{BUSTUB_I8_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type TinyIntUnderlyingType = i8;

#[derive(Copy, Debug)]
pub struct TinyIntType {
    pub(in super::super) value: TinyIntUnderlyingType,
    pub(super) len: u32,
}

impl TinyIntType {
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
