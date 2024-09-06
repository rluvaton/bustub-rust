use std::fmt::{Debug, Display, Formatter};
use std::{cmp, ops};
use std::ops::Deref;
use crate::types::Value;

// Every possible SQL type ID
#[derive(Copy, Clone, PartialEq)]
pub enum TypeId {
    INVALID = 0,
    BOOLEAN = 1,
    TINYINT = 2,
    SMALLINT = 3,
    INTEGER = 4,
    BIGINT = 5,
    DECIMAL = 6,
    VARCHAR = 7,
    TIMESTAMP = 8,
}

pub trait TypeIdTrait<'a>:

    Sized +
    Clone +
    Deref +

    Display +
    Debug +

    // Deserialize a value of the given type from the given storage space.
    From<&'a [u8]> +

    // Serialize this value into the given storage space.
    Into<&'a [u8]> +

    // TODO - add cast as
    // TryInto<dyn TypeIdTrait> +

    // Comparison functions
    // Not using Eq as float number do not implement that
    cmp::PartialEq + // == and !=
    cmp::PartialEq<Value> + // == and !=
    cmp::PartialOrd + // used to derive min, max, and all compare functions
    cmp::PartialOrd<Value> + // used to derive min, max, and all compare functions


    // Other mathematical functions
    ops::Add + // '+'
    ops::Add<Value> + // '+'
    ops::Sub + // '-'
    ops::Sub<Value> + // '-'
    ops::Mul + // '*'
    ops::Mul<Value> + // '*'
    ops::Div + // '/'
    ops::Div<Value> + // '/'
    ops::Rem + // '%'
    ops::Rem<Value> // '%'

{
    fn get_type_id() -> TypeId;

    // TODO - should take ref?
    fn sqrt(self) -> Self {
        // self * self
        todo!()
    }

    // TODO - should return different value?
    fn operate_null(&self, rhs: &Self) -> Self;

    fn is_zero(&self) -> bool;

    // Is the data inlined into this classes storage space, or must it be accessed
    // through an indirection/pointer?
    fn is_inlined(&self) -> bool;

    /// Access the raw variable length data
    fn get_data(&self) -> &[u8];

    // Get the length of the variable length data
    fn get_length(&self) -> u32;

    /// Access the raw varlen data stored from the tuple storage
    fn get_data_from_slice(storage: &[u8]) -> &[u8];

    // Return a stringified version of this value
    fn to_string(&self) -> String;

    // TODO - add more?
}

impl TypeId {
    /// Return the size in bytes of the type
    pub fn get_size(&self) -> u8 {
        match self {
            TypeId::INVALID => unreachable!("Cannot get size of invalid type"),
            TypeId::BOOLEAN => 1,
            TypeId::TINYINT => 1,
            TypeId::SMALLINT => 2,
            TypeId::INTEGER => 4,
            TypeId::BIGINT => 8,
            TypeId::DECIMAL => 8,
            TypeId::TIMESTAMP => 8,

            // TODO - confirm this
            TypeId::VARCHAR => 12,
        }
    }

    pub fn is_coercable_from(&self, from: &TypeId) -> bool {
        match self {
            TypeId::INVALID => false,
            TypeId::BOOLEAN => true,
            TypeId::TINYINT |
            TypeId::SMALLINT |
            TypeId::INTEGER |
            TypeId::BIGINT |
            TypeId::DECIMAL => {
                match from {
                    TypeId::TINYINT |
                    TypeId::SMALLINT |
                    TypeId::INTEGER |
                    TypeId::BIGINT |
                    TypeId::DECIMAL |
                    TypeId::VARCHAR => true,
                    _ => false
                }
            }
            TypeId::TIMESTAMP => {
                matches!(from, TypeId::VARCHAR) || matches!(from, TypeId::TIMESTAMP)
            }

            // TODO - confirm this
            TypeId::VARCHAR => {
                match from {
                    TypeId::BOOLEAN |
                    TypeId::TINYINT |
                    TypeId::SMALLINT |
                    TypeId::INTEGER |
                    TypeId::BIGINT |
                    TypeId::DECIMAL |
                    TypeId::TIMESTAMP |
                    TypeId::VARCHAR => true,
                    _ => false
                }
            }

            _ => self == from
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            TypeId::INVALID => "INVALID",
            TypeId::BOOLEAN => "BOOLEAN",
            TypeId::TINYINT => "TINYINT",
            TypeId::SMALLINT => "SMALLINT",
            TypeId::INTEGER => "INTEGER",
            TypeId::BIGINT => "BIGINT",
            TypeId::DECIMAL => "DECIMAL",
            TypeId::VARCHAR => "VARCHAR",
            TypeId::TIMESTAMP => "TIMESTAMP",
        }
    }

    // Deserialize a value of the given type from the given storage space.
    pub fn deserialize_from(&self, storage: &[u8]) -> Value {
        unimplemented!()
    }
}

impl Debug for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}

impl Display for TypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}
