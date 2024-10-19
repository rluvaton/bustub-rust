use crate::{DBTypeId, DBTypeIdImpl, Value};
use std::fmt::{Debug, Display};
use std::ops;

pub trait ArithmeticsDBTypeTrait:
Sized +
ops::Add<Self> + // '+'
ops::Add<Value, Output=Value> + // '+'
ops::Sub<Self> + // '-'
ops::Sub<Value, Output=Value> + // '-'
ops::Mul<Self> + // '*'
ops::Mul<Value, Output=Value> + // '*'
ops::Div<Self> + // '/'
ops::Div<Value, Output=Value> + // '/'
ops::Rem<Self> + // '%'
ops::Rem<Value, Output=Value> // '%'
{
    fn operate_null(&self, rhs: &Value) -> error_utils::anyhow::Result<Value>;

    unsafe fn operate_null_unchecked(&self, rhs: &Value) -> Value {
        self.operate_null(rhs).expect("Must be able to operate null")
    }
}

pub trait ComparisonDBTypeTrait:
// Not using Eq as float number do not implement that
// PartialEq<Self> + // == and !=
PartialEq<Value> + // == and !=
// PartialOrd<Self> + // used to derive min, max, and all compare functions
PartialOrd<Value> + // used to derive min, max, and all compare functions
Ord
{
    fn get_min_value() -> Self;

    fn get_max_value() -> Self;

    fn is_zero(&self) -> bool;

    fn is_null(&self) -> bool;
}


pub trait ConversionDBTypeTrait:
Into<DBTypeIdImpl> +
Into<Value>
{
    /// Serialize this value into the given storage space.
    fn serialize_to(&self, storage: &mut [u8]);

    /// Deserialize a value of the given type from the given storage space.
    fn deserialize_from(storage: &[u8]) -> Self;

    // Return a stringified version of this value
    fn as_string(&self) -> String;

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl>;

    unsafe fn cast_as_unchecked(&self, db_type_id: DBTypeId) -> DBTypeIdImpl {
        self.try_cast_as(db_type_id).expect("cannot cast as the requested_type")
    }
}

pub trait FormatDBTypeTrait: Display + Debug {
    const NAME: &'static str;

    const TYPE: DBTypeId;
}

pub trait StorageDBTypeTrait: Sized + Clone {
    // Is the data inlined into this classes storage space, or must it be accessed
    // through an indirection/pointer?
    fn is_inlined(&self) -> bool;
}

pub trait VariableLengthStorageDBTypeTrait: Sized + Clone {

    /// Access the raw variable length data
    fn get_data(&self) -> &[u8];

    // Get the length of the variable length data
    fn len(&self) -> u32;

    /// Access the raw varlen data stored from the tuple storage
    fn get_data_from_slice(storage: &[u8]) -> &[u8];
}

pub trait DBTypeIdTrait: FormatDBTypeTrait + ConversionDBTypeTrait + ComparisonDBTypeTrait + ArithmeticsDBTypeTrait + StorageDBTypeTrait {}
