use crate::types::{TimestampType, StorageDBTypeTrait};
use super::TimestampUnderlyingType;


impl Clone for TimestampType {
    fn clone(&self) -> Self {
        TimestampType::new(self.value)
    }
}

impl StorageDBTypeTrait for TimestampType {
    const SIZE: u64 = size_of::<TimestampUnderlyingType>() as u64;

    fn is_inlined(&self) -> bool {
        true
    }

    fn get_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn get_length(&self) -> u32 {
        unimplemented!()
    }

    fn get_data_from_slice(storage: &[u8]) -> &[u8] {
        unimplemented!()
    }
}
