use crate::types::{BUSTUB_TIMESTAMP_MAX, BUSTUB_TIMESTAMP_MIN, BUSTUB_TIMESTAMP_NULL, BUSTUB_VALUE_NULL};
use std::ops::Deref;

pub type TimestampUnderlyingType = u64;

#[derive(Copy, Debug)]
pub struct TimestampType {
    pub(in crate) value: TimestampUnderlyingType,
    pub(super) len: u32,
}

impl TimestampType {
    pub const NULL: TimestampUnderlyingType = BUSTUB_TIMESTAMP_NULL;
    pub const MIN: TimestampUnderlyingType = BUSTUB_TIMESTAMP_MIN;
    pub const MAX: TimestampUnderlyingType = BUSTUB_TIMESTAMP_MAX;

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
