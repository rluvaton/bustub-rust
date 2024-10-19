use crate::types::{BooleanType, StorageDBTypeTrait};


impl Clone for BooleanType {
    fn clone(&self) -> Self {
        BooleanType::new(self.0)
    }
}

impl StorageDBTypeTrait for BooleanType {

    fn is_inlined(&self) -> bool {
        true
    }
}
