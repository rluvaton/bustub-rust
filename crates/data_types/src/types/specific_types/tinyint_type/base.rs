use crate::BUSTUB_VALUE_NULL;
use std::ops::Deref;

pub type TinyIntUnderlyingType = i8;

#[derive(Copy, Debug)]
pub struct TinyIntType {
    pub(in super::super) value: TinyIntUnderlyingType,
    pub(super) len: u32,
}

impl TinyIntType {
    pub const SIZE: usize = size_of::<TinyIntUnderlyingType>();

    pub const NULL: TinyIntUnderlyingType = TinyIntUnderlyingType::MIN;
    pub const MIN: TinyIntUnderlyingType = TinyIntUnderlyingType::MIN + 1;
    pub const MAX: TinyIntUnderlyingType = TinyIntUnderlyingType::MAX;

    pub fn new(value: TinyIntUnderlyingType) -> Self {
        TinyIntType {
            value,
            len: if value == Self::NULL { BUSTUB_VALUE_NULL } else { 0 },
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
        Self::new(Self::NULL)
    }
}
