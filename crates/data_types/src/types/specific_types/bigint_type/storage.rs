use crate::types::{BigIntType, StorageDBTypeTrait};

impl Clone for BigIntType {
    fn clone(&self) -> Self {
        BigIntType::new(self.0)
    }
}

impl StorageDBTypeTrait for BigIntType {

    fn is_inlined(&self) -> bool {
        true
    }
}
