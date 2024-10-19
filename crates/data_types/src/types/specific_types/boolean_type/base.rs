use crate::ComparisonDBTypeTrait;
use std::ops::Deref;

pub type BooleanUnderlyingType = i8;

#[derive(Copy, Debug)]
pub struct BooleanType(pub(crate) BooleanUnderlyingType);

impl BooleanType {
    pub const SIZE: usize = size_of::<BooleanUnderlyingType>();

    pub const NULL: BooleanUnderlyingType = i8::MIN;
    pub const FALSE: BooleanUnderlyingType = 0;
    pub const TRUE: BooleanUnderlyingType = 1;

    pub fn new(value: BooleanUnderlyingType) -> Self {
        BooleanType(value)
    }

    pub fn get_as_bool(&self) -> Option<bool> {
        if self.is_null() {
            None
        } else if self.0 == Self::TRUE {
            Some(true)
        } else if self.0 == Self::FALSE {
            Some(false)
        } else {
            unreachable!()
        }
    }
}

impl Deref for BooleanType {
    type Target = BooleanUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for BooleanType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
