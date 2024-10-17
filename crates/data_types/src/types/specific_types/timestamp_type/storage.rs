use crate::types::{TimestampType, StorageDBTypeTrait};
use super::TimestampUnderlyingType;


impl Clone for TimestampType {
    fn clone(&self) -> Self {
        TimestampType::new(self.value)
    }
}

impl StorageDBTypeTrait for TimestampType {

    fn is_inlined(&self) -> bool {
        true
    }

    fn get_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn len(&self) -> u32 {
        unimplemented!()
    }

    fn get_data_from_slice(storage: &[u8]) -> &[u8] {
        unimplemented!()
    }
}
