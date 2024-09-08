use std::{cmp, ops};
use std::fmt::{Debug, Display};
use crate::types::{DBTypeId, DBTypeIdImpl, Value};

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
    // TODO - should take ref?
    fn sqrt(self) -> Self {
        // self * self
        todo!()
    }

    fn operate_null(&self, rhs: &Value) -> anyhow::Result<Value>;

    unsafe fn operate_null_unchecked(&self, rhs: &Value) -> Value {
        self.operate_null(rhs).expect("Must be able to operate null")
    }
}

pub trait ComparisonDBTypeTrait:
// Not using Eq as float number do not implement that
// cmp::PartialEq<Self> + // == and !=
cmp::PartialEq<Value> + // == and !=
// cmp::PartialOrd<Self> + // used to derive min, max, and all compare functions
cmp::PartialOrd<Value> + // used to derive min, max, and all compare functions
Ord
{
    fn get_min_value() -> Self;

    fn get_max_value() -> Self;

    fn is_zero(&self) -> bool;

    fn is_null(&self) -> bool;
}


pub trait ConversionDBTypeTrait:

// TODO - add cast as
// TryInto<dyn TypeIdTrait> +
Into<DBTypeIdImpl>
{
    /// Serialize this value into the given storage space.
    fn serialize_to(&self, storage: &mut [u8]);

    /// Deserialize a value of the given type from the given storage space.
    fn deserialize_from(storage: &[u8]) -> Self;

    // Return a stringified version of this value
    fn as_string(&self) -> String;

    fn try_cast_as(&self, db_type_id: DBTypeId) -> anyhow::Result<DBTypeIdImpl>;

    unsafe fn cast_as_unchecked(&self, db_type_id: DBTypeId) -> DBTypeIdImpl {
        self.try_cast_as(db_type_id).expect("cannot cast as the requested_type")
    }
}

pub trait FormatDBTypeTrait: Display + Debug {
    const NAME: &'static str;

    const TYPE: DBTypeId;
}

pub trait StorageDBTypeTrait: Sized + Clone {
    /// Get the size of this data type in bytes
    const SIZE: u64;

    // Is the data inlined into this classes storage space, or must it be accessed
    // through an indirection/pointer?
    fn is_inlined(&self) -> bool;

    /// Access the raw variable length data
    fn get_data(&self) -> &[u8];

    // Get the length of the variable length data
    fn get_length(&self) -> u32;

    /// Access the raw varlen data stored from the tuple storage
    fn get_data_from_slice(storage: &[u8]) -> &[u8];
}

pub trait DBTypeIdTrait: FormatDBTypeTrait + ConversionDBTypeTrait + ComparisonDBTypeTrait + ArithmeticsDBTypeTrait + StorageDBTypeTrait {}
