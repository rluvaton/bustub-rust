use crate::{BUSTUB_VALUE_NULL, DBL_LOWEST, FLT_LOWEST};
use std::ops::Deref;

pub type DecimalUnderlyingType = f64;

#[derive(Copy, Debug)]
pub struct DecimalType {
    pub(in super::super) value: DecimalUnderlyingType,
    pub(super) len: u32,
}

impl DecimalType {
    pub const NULL: DecimalUnderlyingType = DBL_LOWEST;
    pub const MIN: DecimalUnderlyingType = FLT_LOWEST;
    pub const MAX: DecimalUnderlyingType = DecimalUnderlyingType::MAX;

    pub fn new(value: DecimalUnderlyingType) -> Self {
        DecimalType {
            value,
            len: if value == Self::NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for DecimalType {
    type Target = DecimalUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for DecimalType {
    fn default() -> Self {
        DecimalType::new(Self::NULL)
    }
}
