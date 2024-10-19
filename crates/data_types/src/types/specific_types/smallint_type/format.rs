use crate::types::{DBTypeId, FormatDBTypeTrait, SmallIntType};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};

impl Display for SmallIntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for SmallIntType {
    const NAME: &'static str = "SMALLINT";
    const TYPE: DBTypeId = DBTypeId::SMALLINT;
}
