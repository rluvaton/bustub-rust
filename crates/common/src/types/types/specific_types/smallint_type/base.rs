use crate::types::{BUSTUB_I16_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type SmallIntUnderlyingType = i16;

#[derive(Copy, Debug)]
pub struct SmallIntType {
    pub(in super::super) value: SmallIntUnderlyingType,
    pub(super) len: u32,
}

impl SmallIntType {
    pub fn new(value: SmallIntUnderlyingType) -> Self {
        SmallIntType {
            value,
            len: if value == BUSTUB_I16_NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for SmallIntType {
    type Target = SmallIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
