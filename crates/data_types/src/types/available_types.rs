use std::fmt::{Debug, Display, Formatter};
use crate::{BigIntType, BooleanType, DBTypeIdImpl, DecimalType, FormatDBTypeTrait, IntType, SmallIntType, StorageDBTypeTrait, TimestampType, TinyIntType, Value};

// Every possible SQL type ID
#[derive(Copy, Clone, PartialEq)]
pub enum DBTypeId {
    INVALID = 0,
    BOOLEAN = 1,
    TINYINT = 2,
    SMALLINT = 3,
    INT = 4,
    BIGINT = 5,
    DECIMAL = 6,
    VARCHAR = 7,
    TIMESTAMP = 8,
}

impl DBTypeId {
    /// Return the size in bytes of the type
    pub fn get_size(&self) -> u64 {
        // TODO - change this to the type
        match self {
            DBTypeId::INVALID => unreachable!("Cannot get size of invalid type"),
            DBTypeId::BOOLEAN => BooleanType::SIZE,
            DBTypeId::TINYINT => TinyIntType::SIZE,
            DBTypeId::SMALLINT => SmallIntType::SIZE,
            DBTypeId::INT => IntType::SIZE,
            DBTypeId::BIGINT => BigIntType::SIZE,
            DBTypeId::DECIMAL => DecimalType::SIZE,
            DBTypeId::TIMESTAMP => TimestampType::SIZE,

            // TODO - confirm this
            DBTypeId::VARCHAR => 12,
        }
    }

    pub fn is_coercable_from(&self, from: &DBTypeId) -> bool {
        match self {
            DBTypeId::INVALID => false,
            DBTypeId::BOOLEAN => true,
            DBTypeId::TINYINT |
            DBTypeId::SMALLINT |
            DBTypeId::INT |
            DBTypeId::BIGINT |
            DBTypeId::DECIMAL => {
                match from {
                    DBTypeId::TINYINT |
                    DBTypeId::SMALLINT |
                    DBTypeId::INT |
                    DBTypeId::BIGINT |
                    DBTypeId::DECIMAL |
                    DBTypeId::VARCHAR => true,
                    _ => false
                }
            }
            DBTypeId::TIMESTAMP => {
                matches!(from, DBTypeId::VARCHAR) || matches!(from, DBTypeId::TIMESTAMP)
            }

            // TODO - confirm this
            DBTypeId::VARCHAR => {
                match from {
                    DBTypeId::BOOLEAN |
                    DBTypeId::TINYINT |
                    DBTypeId::SMALLINT |
                    DBTypeId::INT |
                    DBTypeId::BIGINT |
                    DBTypeId::DECIMAL |
                    DBTypeId::TIMESTAMP |
                    DBTypeId::VARCHAR => true,
                    _ => false
                }
            }

            _ => self == from
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            DBTypeId::INVALID => "INVALID",
            DBTypeId::BOOLEAN => BooleanType::NAME,
            DBTypeId::TINYINT => TinyIntType::NAME,
            DBTypeId::SMALLINT => SmallIntType::NAME,
            DBTypeId::INT => IntType::NAME,
            DBTypeId::BIGINT => BigIntType::NAME,
            DBTypeId::DECIMAL => DecimalType::NAME,
            DBTypeId::VARCHAR => "VARCHAR",
            DBTypeId::TIMESTAMP => TimestampType::NAME,
        }
    }

    // Deserialize a value of the given type from the given storage space.
    pub fn deserialize_from(&self, storage: &[u8]) -> Value {
        unimplemented!()
    }

    pub fn check_comparable(&self, other_type_id: &Self) -> bool {
        match self {
            DBTypeId::BOOLEAN => {
                match other_type_id {
                    DBTypeId::BOOLEAN | DBTypeId::VARCHAR => true,
                    _ => false
                }
            },

            DBTypeId::TINYINT |
            DBTypeId::SMALLINT |
            DBTypeId::INT |
            DBTypeId::BIGINT |
            DBTypeId::DECIMAL => {
                match other_type_id {
                    DBTypeId::TINYINT |
                    DBTypeId::SMALLINT |
                    DBTypeId::INT |
                    DBTypeId::BIGINT |
                    DBTypeId::DECIMAL |
                    DBTypeId::VARCHAR => true,
                    _ => false
                }
            }

            // Anything can be cast to a string!
            DBTypeId::VARCHAR => true,
            DBTypeId::TIMESTAMP => false,
            _ => false
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            DBTypeId::TINYINT |
            DBTypeId::SMALLINT |
            DBTypeId::INT |
            DBTypeId::BIGINT => true,
            _ => false
        }
    }
}

impl Debug for DBTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}

impl Display for DBTypeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name())
    }
}

impl From<DBTypeIdImpl> for DBTypeId {
    fn from(value: DBTypeIdImpl) -> Self {
        value.db_type_id()
    }
}
