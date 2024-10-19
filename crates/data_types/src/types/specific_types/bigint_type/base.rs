use crate::ComparisonDBTypeTrait;
use std::ops::Deref;

pub type BigIntUnderlyingType = i64;

#[derive(Copy, Debug)]
pub struct BigIntType(pub(crate) BigIntUnderlyingType);

impl BigIntType {
    pub const SIZE: usize = size_of::<BigIntUnderlyingType>();

    pub const NULL: BigIntUnderlyingType = BigIntUnderlyingType::MIN;
    pub const MIN: BigIntUnderlyingType = BigIntUnderlyingType::MIN + 1;
    pub const MAX: BigIntUnderlyingType = BigIntUnderlyingType::MAX;

    pub fn new(value: BigIntUnderlyingType) -> Self {
        BigIntType(value)
    }

    pub fn get_as_i64(&self) -> Option<i64> {
        if self.is_null() {
            None
        } else {
            Some(self.0)
        }
    }
}

impl Deref for BigIntType {
    type Target = BigIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for BigIntType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
