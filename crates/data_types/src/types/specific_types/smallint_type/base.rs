use std::ops::Deref;

pub type SmallIntUnderlyingType = i16;

#[derive(Copy, Debug)]
pub struct SmallIntType(pub(crate) SmallIntUnderlyingType);

impl SmallIntType {
    pub const SIZE: usize = size_of::<SmallIntUnderlyingType>();

    pub const NULL: SmallIntUnderlyingType = SmallIntUnderlyingType::MIN;
    pub const MIN: SmallIntUnderlyingType = SmallIntUnderlyingType::MIN + 1;
    pub const MAX: SmallIntUnderlyingType = SmallIntUnderlyingType::MAX;

    pub fn new(value: SmallIntUnderlyingType) -> Self {
        SmallIntType(value)
    }
}

impl Deref for SmallIntType {
    type Target = SmallIntUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for SmallIntType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
