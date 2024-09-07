use crate::types::{BigIntType, DBTypeId, DecimalType, IntType, SmallIntType, TinyIntType};
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
            DBTypeIdImpl::INT($name) => $func,
            DBTypeIdImpl::SMALLINT($name) => $func,
            DBTypeIdImpl::TINYINT($name) => $func,
            DBTypeIdImpl::DECIMAL($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

/// Macro to run the provided expression on the enum variant
/// # Example
///
/// ```
///    use common::run_on_numeric_impl;
/// use common::types::{BigIntType, DBTypeIdImpl};
/// let e = DBTypeIdImpl::BIGINT(BigIntType::new(1));
///
///     // Apply the macro to run trait_function on the enum's variant
///     run_on_numeric_impl!(e, v, {
///         v.cmp(&3);
///     },
///     _ => unreachable!());
/// ```
#[macro_export]
macro_rules! run_on_numeric_impl {
    ($enum_val:expr, $name:ident, $func:expr, $($rest_of_patterns:pat => $value:expr),+) => {
        match $enum_val {
            DBTypeIdImpl::BIGINT($name) => $func,
            DBTypeIdImpl::INT($name) => $func,
            DBTypeIdImpl::SMALLINT($name) => $func,
            DBTypeIdImpl::TINYINT($name) => $func,
            DBTypeIdImpl::DECIMAL($name) => $func,
            // Add match arms for other variants as necessary
            $($rest_of_patterns => $value),+,
        }
    };
}

// Every possible SQL type ID
#[derive(Clone)]
pub enum DBTypeIdImpl {
    // INVALID = 0,
    // BOOLEAN = 1,
    TINYINT(TinyIntType),
    SMALLINT(SmallIntType),
    INT(IntType),
    BIGINT(BigIntType),
    DECIMAL(DecimalType),
    // VARCHAR = 7,
    // TIMESTAMP = 8,
}

impl DBTypeIdImpl {
    pub fn db_type_id(&self) -> DBTypeId {
        match self {
            DBTypeIdImpl::BIGINT(_) => DBTypeId::BIGINT,
            DBTypeIdImpl::INT(_) => DBTypeId::INT,
            DBTypeIdImpl::SMALLINT(_) => DBTypeId::SMALLINT,
            DBTypeIdImpl::TINYINT(_) => DBTypeId::TINYINT,
            DBTypeIdImpl::DECIMAL(_) => DBTypeId::DECIMAL,
        }
    }
}
