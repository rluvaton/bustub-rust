use crate::types::{DBTypeId, FormatDBTypeTrait, TimestampType};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};

impl Display for TimestampType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for TimestampType {
    const NAME: &'static str = "TIMESTAMP";
    const TYPE: DBTypeId = DBTypeId::TIMESTAMP;
}
