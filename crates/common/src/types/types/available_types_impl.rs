use crate::types::{BigIntType, DBTypeId, SmallIntType};
use std::fmt::{Debug, Display};

/// Macro to run the provided expression on the enum variant
/// # Example
///
/// ```
///    use common::run_on_impl;
/// use common::types::{BigIntType, DBTypeIdImpl};
/// let e = DBTypeIdImpl::BIGINT(BigIntType::new(1));
///
///     // Apply the macro to run trait_function on the enum's variant
///     run_on_impl!(e, v, {
///         v.cmp(&3);
///     });
/// ```
#[macro_export]
macro_rules! run_on_impl {
    ($enum_val:expr, $name:ident, $func:expr) => {
        match $enum_val {
            DBTypeIdImpl::BIGINT($name) => $func,
            // DBTypeIdImpl::SMALLINT($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

// Every possible SQL type ID
#[derive(Clone)]
pub enum DBTypeIdImpl {
    // INVALID = 0,
    // BOOLEAN = 1,
    // TINYINT = 2,
    // SMALLINT(SmallIntType),
    // INTEGER = 4,
    BIGINT(BigIntType),
    // DECIMAL = 6,
    // VARCHAR = 7,
    // TIMESTAMP = 8,
}

impl DBTypeIdImpl {
    pub fn db_type_id(&self) -> DBTypeId {
        match self {
            DBTypeIdImpl::BIGINT(_) => DBTypeId::BIGINT,
        }
    }
}
