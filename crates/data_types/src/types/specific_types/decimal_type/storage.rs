use crate::types::{DecimalType, StorageDBTypeTrait};


impl Clone for DecimalType {
    fn clone(&self) -> Self {
        DecimalType::new(self.0)
    }
}

impl StorageDBTypeTrait for DecimalType {

    fn is_inlined(&self) -> bool {
        true
    }
}
