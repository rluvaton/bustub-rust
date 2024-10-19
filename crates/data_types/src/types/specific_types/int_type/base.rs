use std::ops::Deref;

pub type IntUnderlyingType = i32;

#[derive(Copy, Debug)]
pub struct IntType(pub(crate) IntUnderlyingType);

impl IntType {
    pub const SIZE: usize = size_of::<IntUnderlyingType>();

    pub const NULL: IntUnderlyingType = IntUnderlyingType::MIN;
    pub const MIN: IntUnderlyingType = IntUnderlyingType::MIN + 1;
    pub const MAX: IntUnderlyingType = IntUnderlyingType::MAX;

    pub fn new(value: IntUnderlyingType) -> Self {
        IntType(value)
    }
}

impl Deref for IntType {
    type Target = IntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for IntType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
