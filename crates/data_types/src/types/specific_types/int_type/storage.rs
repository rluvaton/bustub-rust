use crate::types::{IntType, StorageDBTypeTrait};


impl Clone for IntType {
    fn clone(&self) -> Self {
        IntType::new(self.0)
    }
}

impl StorageDBTypeTrait for IntType {

    fn is_inlined(&self) -> bool {
        true
    }
}
