use crate::BUSTUB_VALUE_NULL;
use std::ops::Deref;

pub type TinyIntUnderlyingType = i8;

#[derive(Copy, Debug)]
pub struct TinyIntType(pub(crate) TinyIntUnderlyingType);

impl TinyIntType {
    pub const SIZE: usize = size_of::<TinyIntUnderlyingType>();

    pub const NULL: TinyIntUnderlyingType = TinyIntUnderlyingType::MIN;
    pub const MIN: TinyIntUnderlyingType = TinyIntUnderlyingType::MIN + 1;
    pub const MAX: TinyIntUnderlyingType = TinyIntUnderlyingType::MAX;

    pub fn new(value: TinyIntUnderlyingType) -> Self {
        TinyIntType(value)
    }
}

impl Deref for TinyIntType {
    type Target = TinyIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for TinyIntType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
