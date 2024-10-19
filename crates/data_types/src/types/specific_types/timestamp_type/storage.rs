use crate::types::{StorageDBTypeTrait, TimestampType};


impl Clone for TimestampType {
    fn clone(&self) -> Self {
        TimestampType::new(self.0)
    }
}

impl StorageDBTypeTrait for TimestampType {

    fn is_inlined(&self) -> bool {
        true
    }
}
