use crate::types::{DecimalType, DecimalUnderlyingType, StorageDBTypeTrait};


impl Clone for DecimalType {
    fn clone(&self) -> Self {
        DecimalType::new(self.value)
    }
}

impl StorageDBTypeTrait for DecimalType {
    const SIZE: u64 = size_of::<DecimalUnderlyingType>() as u64;

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
