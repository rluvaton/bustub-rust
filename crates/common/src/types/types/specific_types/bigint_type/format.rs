use std::fmt::{Display, Formatter};
use crate::types::{BigIntType, DBTypeId};
use crate::types::types::specific_types_trait::FormatDBTypeTrait;


impl Display for BigIntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for BigIntType {
    const NAME: &'static str = "BIGINT";
    const TYPE: DBTypeId = DBTypeId::BIGINT;
}
