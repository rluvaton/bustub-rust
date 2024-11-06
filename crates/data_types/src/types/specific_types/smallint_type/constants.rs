use crate::SmallIntType;
use crate::types::specific_types_trait::ConstantsDBTypeTrait;

impl ConstantsDBTypeTrait for SmallIntType {
    fn get_null() -> Self {
        SmallIntType::from(None)
    }
}
