use crate::types::{BooleanType, BooleanUnderlyingType, StorageDBTypeTrait};


impl Clone for BooleanType {
    fn clone(&self) -> Self {
        BooleanType::new(self.value)
    }
}

impl StorageDBTypeTrait for BooleanType {
    const SIZE: u64 = size_of::<BooleanUnderlyingType>() as u64;

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
