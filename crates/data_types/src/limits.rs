
pub const DBL_LOWEST: f64 = f64::MIN;
pub const FLT_LOWEST: f64 = f32::MIN as f64;

pub const BUSTUB_DATE_MIN: u32 = 0;

pub const BUSTUB_U64_MAX: u64 = u64::MAX - 1;
pub const BUSTUB_DATE_MAX: u64 = i32::MAX as u64;

pub const BUSTUB_VALUE_NULL: u32 = u32::MAX;
pub const BUSTUB_DATE_NULL: u64 = 0;

pub const BUSTUB_VARCHAR_MAX_LEN: u32 = u32::MAX;

// Use to make TEXT type as the alias of VARCHAR(TEXT_MAX_LENGTH)
pub const BUSTUB_TEXT_MAX_LEN: u32 = 1000000000;

// Objects (i.e., VARCHAR) with length prefix of -1 are NULL
pub const OBJECTLENGTH_NULL: i32 = -1;
