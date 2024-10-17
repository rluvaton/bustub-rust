use crate::{BUSTUB_VALUE_NULL, BUSTUB_VARCHAR_MAX_LEN};
use std::ops::Deref;

// Should it be string or Box<str>?
pub type VarcharUnderlyingType = String;

#[derive(Clone, Debug)]
pub struct VarcharType {
    pub(crate) value: String,
    pub(super) len: u32,
}

impl VarcharType {
    pub fn new(value: Option<&str>) -> Self {
        if let Some(value) = value {
            Self::assert_valid_before_creation(value);
            VarcharType {
                len: value.len() as u32,
                value: value.to_string(),
            }
        } else {
            VarcharType {
                len: BUSTUB_VALUE_NULL,
                value: "".to_string(),
            }
        }
    }

    #[inline(always)]
    pub(super) fn assert_valid_before_creation(value: &str) {
        assert!(value.len() < BUSTUB_VARCHAR_MAX_LEN as usize, "String length must be below {}", BUSTUB_VARCHAR_MAX_LEN);
    }
}

impl Deref for VarcharType {
    type Target = VarcharUnderlyingType;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Default for VarcharType {
    fn default() -> Self {
        Self::new(None)
    }
}
