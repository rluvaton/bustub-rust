use crate::types::{BooleanType, DBTypeId, FormatDBTypeTrait};
use std::fmt::{Display, Formatter};


impl Display for BooleanType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for BooleanType {
    const NAME: &'static str = "BOOLEAN";
    const TYPE: DBTypeId = DBTypeId::BOOLEAN;
}
