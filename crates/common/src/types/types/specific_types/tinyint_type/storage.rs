use crate::types::{TinyIntType, StorageDBTypeTrait};
use super::TinyIntUnderlyingType;


impl Clone for TinyIntType {
    fn clone(&self) -> Self {
        TinyIntType::new(self.value)
    }
}

impl StorageDBTypeTrait for TinyIntType {
    const SIZE: u64 = size_of::<TinyIntUnderlyingType>() as u64;

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
