
pub const DBL_LOWEST: f64 = f64::MIN;
pub const FLT_LOWEST: f64 = f32::MIN as f64;

pub const BUSTUB_I8_MIN: i8 = i8::MIN + 1;
pub const BUSTUB_I16_MIN: i16 = (i16::MIN + 1);
pub const BUSTUB_I32_MIN: i32 = (i32::MIN + 1);
pub const BUSTUB_I64_MIN: i64 = (i64::MIN + 1);
pub const BUSTUB_DECIMAL_MIN: f64 = FLT_LOWEST;
pub const BUSTUB_TIMESTAMP_MIN: u64 = 0;
pub const BUSTUB_DATE_MIN: u32 = 0;
pub const BUSTUB_BOOLEAN_MIN: i8 = 0;

pub const BUSTUB_I8_MAX:  i8 = i8::MAX;
pub const BUSTUB_I16_MAX: i16 = i16::MAX;
pub const BUSTUB_I32_MAX: i32 = i32::MAX;
pub const BUSTUB_I64_MAX: i64 = i64::MAX;
pub const BUSTUB_U64_MAX: u64 = u64::MAX - 1;
pub const BUSTUB_DECIMAL_MAX: f64 = f64::MAX;
pub const BUSTUB_TIMESTAMP_MAX: u64 = 11231999986399999999;
pub const BUSTUB_DATE_MAX: u64 = i32::MAX as u64;
pub const BUSTUB_BOOLEAN_MAX: u8 = 1;

pub const BUSTUB_VALUE_NULL: u32 = u32::MAX;
pub const BUSTUB_INT8_NULL:  i8 = i8::MIN;
pub const BUSTUB_INT16_NULL: i16 = i16::MIN;
pub const BUSTUB_INT32_NULL: i32 = i32::MIN;
pub const BUSTUB_INT64_NULL: i64 = i64::MIN;
pub const BUSTUB_DATE_NULL: u64 = 0;
pub const BUSTUB_TIMESTAMP_NULL: u64 = u64::MAX;
pub const BUSTUB_DECIMAL_NULL: f64 = DBL_LOWEST;
pub const BUSTUB_BOOLEAN_NULL: i8 = i8::MIN;

pub const BUSTUB_VARCHAR_MAX_LEN: u32 = u32::MAX;

// Use to make TEXT type as the alias of VARCHAR(TEXT_MAX_LENGTH)
pub const BUSTUB_TEXT_MAX_LEN: u32 = 1000000000;

// Objects (i.e., VARCHAR) with length prefix of -1 are NULL
pub const OBJECTLENGTH_NULL: i32 = -1;
