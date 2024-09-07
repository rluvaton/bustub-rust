use crate::types::{BUSTUB_DECIMAL_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type DecimalUnderlyingType = f64;

#[derive(Copy, Debug)]
pub struct DecimalType {
    pub(in super::super) value: DecimalUnderlyingType,
    pub(super) len: u32,
}

impl DecimalType {
    pub fn new(value: DecimalUnderlyingType) -> Self {
        DecimalType {
            value,
            len: if value == BUSTUB_DECIMAL_NULL { BUSTUB_VALUE_NULL } else { 0 },
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
        DecimalType::new(BUSTUB_DECIMAL_NULL)
    }
}