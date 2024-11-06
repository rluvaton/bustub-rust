use crate::types::specific_types_trait::ConstantsDBTypeTrait;
use crate::IntType;

impl ConstantsDBTypeTrait for IntType {
    fn get_null() -> Self {
        IntType::from(None)
    }
}
