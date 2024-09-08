use crate::types::{DBTypeId, DecimalType, FormatDBTypeTrait};
use std::fmt::{Display, Formatter};


impl Display for DecimalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FormatDBTypeTrait for DecimalType {
    const NAME: &'static str = "DECIMAL";
    const TYPE: DBTypeId = DBTypeId::DECIMAL;
}
