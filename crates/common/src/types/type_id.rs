use std::fmt::{Debug, Display, Formatter};
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
