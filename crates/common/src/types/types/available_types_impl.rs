use crate::types::{BigIntType, DBTypeId};

// Every possible SQL type ID
pub enum DBTypeIdImpl {
    // INVALID = 0,
    // BOOLEAN = 1,
    // TINYINT = 2,
    // SMALLINT = 3,
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
