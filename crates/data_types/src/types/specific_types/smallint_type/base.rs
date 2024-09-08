use crate::BUSTUB_VALUE_NULL;
use std::ops::Deref;

pub type SmallIntUnderlyingType = i16;

#[derive(Copy, Debug)]
pub struct SmallIntType {
    pub(in super::super) value: SmallIntUnderlyingType,
    pub(super) len: u32,
}

impl SmallIntType {
    pub const NULL: SmallIntUnderlyingType = SmallIntUnderlyingType::MIN;
    pub const MIN: SmallIntUnderlyingType = SmallIntUnderlyingType::MIN + 1;
    pub const MAX: SmallIntUnderlyingType = SmallIntUnderlyingType::MAX;

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