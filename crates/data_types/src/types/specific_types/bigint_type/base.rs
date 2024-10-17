use crate::BUSTUB_VALUE_NULL;
use std::ops::Deref;

pub type BigIntUnderlyingType = i64;

#[derive(Copy, Debug)]
pub struct BigIntType {
    pub(in crate) value: BigIntUnderlyingType,
    pub(super) len: u32,
}

impl BigIntType {
    pub const SIZE: usize = size_of::<BigIntUnderlyingType>();

    pub const NULL: BigIntUnderlyingType = BigIntUnderlyingType::MIN;
    pub const MIN: BigIntUnderlyingType = BigIntUnderlyingType::MIN + 1;
    pub const MAX: BigIntUnderlyingType = BigIntUnderlyingType::MAX;

    pub fn new(value: BigIntUnderlyingType) -> Self {
        BigIntType {
            value,
            len: if value == Self::NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for BigIntType {
    type Target = BigIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for BigIntType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
