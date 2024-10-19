use crate::types::{SmallIntType, StorageDBTypeTrait};


impl Clone for SmallIntType {
    fn clone(&self) -> Self {
        SmallIntType::new(self.0)
    }
}

impl StorageDBTypeTrait for SmallIntType {
    fn is_inlined(&self) -> bool {
        true
    }
}
