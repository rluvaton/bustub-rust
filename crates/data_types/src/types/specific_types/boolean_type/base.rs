use crate::{ComparisonDBTypeTrait, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type BooleanUnderlyingType = i8;

#[derive(Copy, Debug)]
pub struct BooleanType {
    pub(in super::super) value: BooleanUnderlyingType,
    pub(super) len: u32,
}

impl BooleanType {
    pub const NULL: BooleanUnderlyingType = i8::MIN;
    pub const FALSE: BooleanUnderlyingType = 0;
    pub const TRUE: BooleanUnderlyingType = 1;

    pub fn new(value: BooleanUnderlyingType) -> Self {
        BooleanType {
            value,
            len: if value == Self::NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }

    pub fn get_as_bool(&self) -> Option<bool> {
        if self.is_null() {
            None
        } else if self.value == Self::TRUE {
            Some(true)
        } else if self.value == Self::FALSE {
            Some(false)
        } else {
            unreachable!()
        }
    }
}

impl Deref for BooleanType {
    type Target = BooleanUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for BooleanType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
