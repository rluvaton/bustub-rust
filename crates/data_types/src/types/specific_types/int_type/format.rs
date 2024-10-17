use std::fmt::{Display, Formatter};
use crate::types::{IntType, DBTypeId, FormatDBTypeTrait};
use crate::{ConversionDBTypeTrait, VarcharType};

impl Display for IntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for IntType {
    const NAME: &'static str = "INTEGER";
    const TYPE: DBTypeId = DBTypeId::INT;
}
