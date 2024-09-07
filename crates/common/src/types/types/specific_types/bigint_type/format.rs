use std::fmt::{Display, Formatter};
use crate::types::{BigIntType, DBTypeId, FormatDBTypeTrait};


impl Display for BigIntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for BigIntType {
    const NAME: &'static str = "BIGINT";
    const TYPE: DBTypeId = DBTypeId::BIGINT;
}
