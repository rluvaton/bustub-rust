use crate::BUSTUB_VALUE_NULL;
use std::ops::Deref;

// Not using Timestamp from `common::config::Timestamp` on purpose as it is i64 and not u64
pub type TimestampUnderlyingType = u64;

#[derive(Copy, Debug)]
pub struct TimestampType {
    pub(in crate) value: TimestampUnderlyingType,
    pub(super) len: u32,
}

impl TimestampType {
    pub const SIZE: usize = size_of::<TimestampUnderlyingType>();

    pub const NULL: TimestampUnderlyingType = TimestampUnderlyingType::MAX;
    pub const MIN: TimestampUnderlyingType = 0;
    pub const MAX: TimestampUnderlyingType = 11231999986399999999;

    pub fn new(value: TimestampUnderlyingType) -> Self {
        TimestampType {
            value,
            len: if value == Self::NULL { BUSTUB_VALUE_NULL } else { 0 },
        }
    }
}

impl Deref for TimestampType {
    type Target = TimestampUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for TimestampType {
    fn default() -> Self {
        Self::new(Self::NULL)
    }
}
