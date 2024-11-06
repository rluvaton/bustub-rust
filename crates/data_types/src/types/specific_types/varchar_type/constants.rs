use crate::VarcharType;
use crate::types::specific_types_trait::ConstantsDBTypeTrait;

impl ConstantsDBTypeTrait for VarcharType {
    fn get_null() -> Self {
        VarcharType::new(None)
    }
}
