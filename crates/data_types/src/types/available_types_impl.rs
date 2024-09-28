use crate::types::{BigIntType, BooleanType, DBTypeId, DecimalType, IntType, SmallIntType, TimestampType, TinyIntType};

/// Macro to run the provided expression on the enum variant
/// # Example
///
/// ```
/// use data_types::*;
/// let e = DBTypeIdImpl::BIGINT(BigIntType::new(1));
///
/// // Apply the macro to run trait_function on the enum's variant
/// let _ = run_on_impl!(e, v, {
///    v.is_null()
/// });
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
            DBTypeIdImpl::BOOLEAN($name) => $func,
            DBTypeIdImpl::TIMESTAMP($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

/// Macro to run the provided expression on the enum variant
/// # Example
///
/// ```
/// use data_types::*;
/// let e = DBTypeIdImpl::BIGINT(BigIntType::new(1));
///
///  // Apply the macro to run trait_function on the enum's variant
///  let res = run_on_numeric_impl!(e, v, {
///         v.is_null()
///     },
///     _ => unreachable!()
///  );
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
    BOOLEAN(BooleanType),
    TINYINT(TinyIntType),
    SMALLINT(SmallIntType),
    INT(IntType),
    BIGINT(BigIntType),
    DECIMAL(DecimalType),
    // VARCHAR = 7,
    TIMESTAMP(TimestampType),
}

impl DBTypeIdImpl {
    pub fn db_type_id(&self) -> DBTypeId {
        match self {
            DBTypeIdImpl::BOOLEAN(_) => DBTypeId::BOOLEAN,
            DBTypeIdImpl::BIGINT(_) => DBTypeId::BIGINT,
            DBTypeIdImpl::INT(_) => DBTypeId::INT,
            DBTypeIdImpl::SMALLINT(_) => DBTypeId::SMALLINT,
            DBTypeIdImpl::TINYINT(_) => DBTypeId::TINYINT,
            DBTypeIdImpl::DECIMAL(_) => DBTypeId::DECIMAL,
            DBTypeIdImpl::TIMESTAMP(_) => DBTypeId::TIMESTAMP,
        }
    }
}
