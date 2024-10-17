use crate::types::{DBTypeId, DecimalType, FormatDBTypeTrait};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};

impl Display for DecimalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for DecimalType {
    const NAME: &'static str = "DECIMAL";
    const TYPE: DBTypeId = DBTypeId::DECIMAL;
}
