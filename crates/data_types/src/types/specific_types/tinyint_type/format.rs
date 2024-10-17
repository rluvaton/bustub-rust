use crate::types::{DBTypeId, FormatDBTypeTrait, TinyIntType};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};

impl Display for TinyIntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for TinyIntType {
    const NAME: &'static str = "TINYINT";
    const TYPE: DBTypeId = DBTypeId::TINYINT;
}
