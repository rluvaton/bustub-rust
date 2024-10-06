use crate::types::{BigIntType, StorageDBTypeTrait};
use super::BigIntUnderlyingType;


impl Clone for BigIntType {
    fn clone(&self) -> Self {
        BigIntType::new(self.value)
    }
}

impl StorageDBTypeTrait for BigIntType {
    const SIZE: u64 = size_of::<BigIntUnderlyingType>() as u64;

    fn is_inlined(&self) -> bool {
        true
    }

    fn get_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn get_length(&self) -> u32 {
        unimplemented!()
    }

    fn get_data_from_slice(_storage: &[u8]) -> &[u8] {
        unimplemented!()
    }
}
