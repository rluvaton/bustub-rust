use crate::types::{DBTypeId, FormatDBTypeTrait};
use crate::{ConversionDBTypeTrait, VarcharType};
use std::fmt::{Display, Formatter};

impl Display for VarcharType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_string().as_str())
    }
}

impl FormatDBTypeTrait for VarcharType {
    const NAME: &'static str = "VARCHAR";
    const TYPE: DBTypeId = DBTypeId::VARCHAR;
}
