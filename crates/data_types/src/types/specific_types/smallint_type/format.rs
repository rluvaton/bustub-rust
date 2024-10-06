use std::fmt::{Display, Formatter};
use crate::types::{SmallIntType, DBTypeId, FormatDBTypeTrait};


impl Display for SmallIntType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for SmallIntType {
    const NAME: &'static str = "SMALLINT";
    const TYPE: DBTypeId = DBTypeId::SMALLINT;
}
