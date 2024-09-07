use crate::types::{SmallIntType, StorageDBTypeTrait};
use super::SmallIntUnderlyingType;


impl Clone for SmallIntType {
    fn clone(&self) -> Self {
        SmallIntType::new(self.value)
    }
}

impl StorageDBTypeTrait for SmallIntType {
    const SIZE: u64 = size_of::<SmallIntUnderlyingType>() as u64;

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
