use std::fmt::{Display, Formatter};
use crate::types::{BigIntType, DBTypeId, FormatDBTypeTrait, TimestampType};


impl Display for TimestampType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for TimestampType {
    const NAME: &'static str = "TIMESTAMP";
    const TYPE: DBTypeId = DBTypeId::TIMESTAMP;
}
