use std::fmt::{Display, Formatter};
use crate::types::{TinyIntType, DBTypeId, FormatDBTypeTrait};


impl Display for TinyIntType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for TinyIntType {
    const NAME: &'static str = "TINYINT";
    const TYPE: DBTypeId = DBTypeId::TINYINT;
}
