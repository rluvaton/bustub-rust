use crate::{BUSTUB_I16_MAX, BUSTUB_I16_MIN, BUSTUB_I16_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type SmallIntUnderlyingType = i16;

#[derive(Copy, Debug)]
pub struct SmallIntType {
    pub(in super::super) value: SmallIntUnderlyingType,
    pub(super) len: u32,
}

impl SmallIntType {
    pub const NULL: SmallIntUnderlyingType = BUSTUB_I16_NULL;
    pub const MIN: SmallIntUnderlyingType = BUSTUB_I16_MIN;
    pub const MAX: SmallIntUnderlyingType = BUSTUB_I16_MAX;

    pub fn new(value: SmallIntUnderlyingType) -> Self {
        SmallIntType {
            value,
            len: if value == Self::NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for SmallIntType {
    type Target = SmallIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for SmallIntType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
