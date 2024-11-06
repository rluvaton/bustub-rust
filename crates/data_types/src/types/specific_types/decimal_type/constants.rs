use crate::DecimalType;
use crate::types::specific_types_trait::ConstantsDBTypeTrait;

impl ConstantsDBTypeTrait for DecimalType {
    fn get_null() -> Self {
        DecimalType::from(None)
    }
}
