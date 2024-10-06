use std::fmt::{Display, Formatter};
use crate::types::{IntType, DBTypeId, FormatDBTypeTrait};


impl Display for IntType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for IntType {
    const NAME: &'static str = "INTEGER";
    const TYPE: DBTypeId = DBTypeId::INT;
}
