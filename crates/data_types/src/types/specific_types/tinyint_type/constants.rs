use crate::TinyIntType;
use crate::types::specific_types_trait::ConstantsDBTypeTrait;

impl ConstantsDBTypeTrait for TinyIntType {
    fn get_null() -> Self {
        TinyIntType::from(None)
    }
}
