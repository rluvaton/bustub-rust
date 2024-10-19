use crate::types::{StorageDBTypeTrait, TinyIntType};


impl Clone for TinyIntType {
    fn clone(&self) -> Self {
        TinyIntType::new(self.0)
    }
}

impl StorageDBTypeTrait for TinyIntType {

    fn is_inlined(&self) -> bool {
        true
    }
}
