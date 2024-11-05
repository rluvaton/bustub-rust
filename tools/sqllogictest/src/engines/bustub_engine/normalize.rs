use std::ops::Deref;
use super::super::conversion::*;
use data_types::{DBTypeIdImpl, Value};


/// Normalizes the content of a single cell in RecordBatch prior to printing.
///
/// This is to make the output comparable to the semi-standard .slt format
///
/// Normalizations applied to [NULL Values and empty strings]
///
/// [NULL Values and empty strings]: https://duckdb.org/dev/sqllogictest/result_verification#null-values-and-empty-strings
///
/// Floating numbers are rounded to have a consistent representation with the Postgres runner.
///
pub fn cell_to_string(col: &Value) -> String {
    if col.is_null() {
        NULL_STR.to_string()
    } else {
        match col.get_value() {
            DBTypeIdImpl::BOOLEAN(t) => bool_to_str(t.get_as_bool()),
            DBTypeIdImpl::DECIMAL(t) => f64_to_str(*t.deref()),
            DBTypeIdImpl::VARCHAR(t) => varchar_to_str(t.as_str()),
            DBTypeIdImpl::SMALLINT(t) => t.to_string(),
            DBTypeIdImpl::TINYINT(t) => t.to_string(),
            DBTypeIdImpl::INT(t) => t.to_string(),
            DBTypeIdImpl::BIGINT(t) => t.to_string(),
            DBTypeIdImpl::TIMESTAMP(t) => t.to_string(),
        }
    }
}
