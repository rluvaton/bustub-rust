use crate::types::{BooleanType, DBTypeId, FormatDBTypeTrait};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};

impl Display for BooleanType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for BooleanType {
    const NAME: &'static str = "BOOLEAN";
    const TYPE: DBTypeId = DBTypeId::BOOLEAN;
}
