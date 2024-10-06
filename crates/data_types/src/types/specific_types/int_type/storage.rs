use crate::types::{IntType, IntUnderlyingType, StorageDBTypeTrait};


impl Clone for IntType {
    fn clone(&self) -> Self {
        IntType::new(self.value)
    }
}

impl StorageDBTypeTrait for IntType {
    const SIZE: u64 = size_of::<IntUnderlyingType>() as u64;

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
