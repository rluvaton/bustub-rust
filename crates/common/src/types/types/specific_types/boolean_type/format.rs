use std::fmt::{Display, Formatter};
use crate::types::{TinyIntType, DBTypeId, FormatDBTypeTrait, BooleanType};


impl Display for BooleanType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for BooleanType {
    const NAME: &'static str = "BOOLEAN";
    const TYPE: DBTypeId = DBTypeId::BOOLEAN;
}
