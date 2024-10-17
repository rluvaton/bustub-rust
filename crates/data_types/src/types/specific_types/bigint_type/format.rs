use crate::types::{BigIntType, DBTypeId, FormatDBTypeTrait};
use crate::ConversionDBTypeTrait;
use std::fmt::{Display, Formatter};


impl Display for BigIntType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for BigIntType {
    const NAME: &'static str = "BIGINT";
    const TYPE: DBTypeId = DBTypeId::BIGINT;
}
