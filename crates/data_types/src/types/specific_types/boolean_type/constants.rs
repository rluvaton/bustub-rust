use crate::BooleanType;
use crate::types::specific_types_trait::ConstantsDBTypeTrait;

impl ConstantsDBTypeTrait for BooleanType {
    fn get_null() -> Self {
        BooleanType::from(None)
    }
}
