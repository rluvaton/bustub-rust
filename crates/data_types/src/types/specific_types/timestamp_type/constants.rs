use crate::types::specific_types_trait::ConstantsDBTypeTrait;
use crate::TimestampType;

impl ConstantsDBTypeTrait for TimestampType {
    fn get_null() -> Self {
        TimestampType::from(None)
    }
}
