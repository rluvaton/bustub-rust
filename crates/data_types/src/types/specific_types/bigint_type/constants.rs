use crate::{BigIntType};
use crate::types::specific_types_trait::ConstantsDBTypeTrait;

impl ConstantsDBTypeTrait for BigIntType {
    fn get_null() -> Self {
        BigIntType::from(None)
    }
}
