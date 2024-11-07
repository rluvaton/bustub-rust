use std::fmt::{Debug, Display, Formatter};
use crate::{BigIntType, BooleanType, DBTypeIdImpl, DecimalType, FormatDBTypeTrait, IntType, SmallIntType, TimestampType, TinyIntType, Value};

// Every possible SQL type ID
#[derive(Copy, Clone, PartialEq, Eq)]
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

pub enum CanBeCastedWithoutValueChangeResult {
    True,
    NeedNumberBoundCheck,
    NeedVarLengthCheck,
    False,
}

impl DBTypeId {
    /// Return the size in bytes of the type
    pub fn get_size(&self) -> usize {
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

    /// Cant cast to another type
    /// (can change the actual value - e.g. casting to varchar will make the value string)
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
        }
    }

    /// Can type be cast as the provided type without value changes
    pub fn can_be_cast_without_value_changes(&self, cast_as: &DBTypeId) -> CanBeCastedWithoutValueChangeResult {
        match (self, cast_as) {
            (DBTypeId::INVALID, DBTypeId::INVALID) => CanBeCastedWithoutValueChangeResult::True,
            (DBTypeId::INVALID, _) | (_, DBTypeId::INVALID) => CanBeCastedWithoutValueChangeResult::False,

            (DBTypeId::BOOLEAN, DBTypeId::BOOLEAN) => CanBeCastedWithoutValueChangeResult::True,
            (DBTypeId::BOOLEAN, _) | (_, DBTypeId::BOOLEAN) => CanBeCastedWithoutValueChangeResult::False,

            // Varchar need bound check for the length
            (DBTypeId::VARCHAR, DBTypeId::VARCHAR) => CanBeCastedWithoutValueChangeResult::NeedVarLengthCheck,
            (DBTypeId::VARCHAR, _) | (_, DBTypeId::VARCHAR) => CanBeCastedWithoutValueChangeResult::False,

            (DBTypeId::TIMESTAMP, DBTypeId::TIMESTAMP) => CanBeCastedWithoutValueChangeResult::True,
            (DBTypeId::TIMESTAMP, _) | (_, DBTypeId::TIMESTAMP) => CanBeCastedWithoutValueChangeResult::False,

            (DBTypeId::TINYINT, DBTypeId::TINYINT) |
            (DBTypeId::TINYINT, DBTypeId::SMALLINT) |
            (DBTypeId::TINYINT, DBTypeId::INT) |
            (DBTypeId::TINYINT, DBTypeId::BIGINT) |
            (DBTypeId::TINYINT, DBTypeId::DECIMAL) |

            (DBTypeId::SMALLINT, DBTypeId::SMALLINT) |
            (DBTypeId::SMALLINT, DBTypeId::INT) |
            (DBTypeId::SMALLINT, DBTypeId::BIGINT) |
            (DBTypeId::SMALLINT, DBTypeId::DECIMAL) |

            (DBTypeId::INT, DBTypeId::INT) |
            (DBTypeId::INT, DBTypeId::BIGINT) |
            (DBTypeId::INT, DBTypeId::DECIMAL) |

            (DBTypeId::BIGINT, DBTypeId::BIGINT) |
            (DBTypeId::BIGINT, DBTypeId::DECIMAL) |

            (DBTypeId::DECIMAL, DBTypeId::DECIMAL) => CanBeCastedWithoutValueChangeResult::True,

            (DBTypeId::SMALLINT, DBTypeId::TINYINT) |

            (DBTypeId::INT, DBTypeId::TINYINT) |
            (DBTypeId::INT, DBTypeId::SMALLINT) |

            (DBTypeId::BIGINT, DBTypeId::TINYINT) |
            (DBTypeId::BIGINT, DBTypeId::SMALLINT) |
            (DBTypeId::BIGINT, DBTypeId::INT) |

            (DBTypeId::DECIMAL, DBTypeId::TINYINT) |
            (DBTypeId::DECIMAL, DBTypeId::SMALLINT) |
            (DBTypeId::DECIMAL, DBTypeId::INT) |
            (DBTypeId::DECIMAL, DBTypeId::BIGINT) => CanBeCastedWithoutValueChangeResult::NeedNumberBoundCheck
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
    pub fn deserialize_from(&self, _storage: &[u8]) -> Value {
        unimplemented!()
    }

    pub fn check_comparable(&self, other_type_id: &Self) -> bool {
        match self {
            DBTypeId::BOOLEAN => {
                match other_type_id {
                    DBTypeId::BOOLEAN | DBTypeId::VARCHAR => true,
                    _ => false
                }
            }

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
