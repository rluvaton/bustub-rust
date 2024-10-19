use crate::types::{DBTypeId, FormatDBTypeTrait, IntType};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};

impl Display for IntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for IntType {
    const NAME: &'static str = "INTEGER";
    const TYPE: DBTypeId = DBTypeId::INT;
}
