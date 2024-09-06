use std::fmt::{Debug, Display, Formatter};
use crate::types::{BigIntType, DBTypeIdImpl, TypeIdTrait, Value};
use crate::types::DBTypeId::BIGINT;

// Every possible SQL type ID
#[derive(Copy, Clone, PartialEq)]
pub enum DBTypeId {
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

impl DBTypeId {
    /// Return the size in bytes of the type
    pub fn get_size(&self) -> u64 {
        // TODO - change this to the type
        match self {
            DBTypeId::INVALID => unreachable!("Cannot get size of invalid type"),
            DBTypeId::BOOLEAN => 1,
            DBTypeId::TINYINT => 1,
            DBTypeId::SMALLINT => 2,
            DBTypeId::INTEGER => 4,
            DBTypeId::BIGINT => BigIntType::SIZE,
            DBTypeId::DECIMAL => 8,
            DBTypeId::TIMESTAMP => 8,

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
            DBTypeId::INTEGER |
            DBTypeId::BIGINT |
            DBTypeId::DECIMAL => {
                match from {
                    DBTypeId::TINYINT |
                    DBTypeId::SMALLINT |
                    DBTypeId::INTEGER |
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
                    DBTypeId::INTEGER |
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
            DBTypeId::BOOLEAN => "BOOLEAN",
            DBTypeId::TINYINT => "TINYINT",
            DBTypeId::SMALLINT => "SMALLINT",
            DBTypeId::INTEGER => "INTEGER",
            DBTypeId::BIGINT => BigIntType::NAME,
            DBTypeId::DECIMAL => "DECIMAL",
            DBTypeId::VARCHAR => "VARCHAR",
            DBTypeId::TIMESTAMP => "TIMESTAMP",
        }
    }

    // Deserialize a value of the given type from the given storage space.
    pub fn deserialize_from(&self, storage: &[u8]) -> Value {
        unimplemented!()
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
