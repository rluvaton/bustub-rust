use crate::{DBL_LOWEST, FLT_LOWEST};
use std::ops::Deref;

pub type DecimalUnderlyingType = f64;

#[derive(Copy, Debug)]
pub struct DecimalType(pub(crate) DecimalUnderlyingType);

impl DecimalType {
    pub const SIZE: usize = size_of::<DecimalUnderlyingType>();

    pub const NULL: DecimalUnderlyingType = DBL_LOWEST;
    pub const MIN: DecimalUnderlyingType = FLT_LOWEST;
    pub const MAX: DecimalUnderlyingType = DecimalUnderlyingType::MAX;

    pub fn new(value: DecimalUnderlyingType) -> Self {
        DecimalType(value)
    }
}

impl Deref for DecimalType {
    type Target = DecimalUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for DecimalType {
    fn default() -> Self {
        DecimalType::new(Self::NULL)
    }
}
