use crate::types::{IntType, StorageDBTypeTrait};


impl Clone for IntType {
    fn clone(&self) -> Self {
        IntType::new(self.value)
    }
}

impl StorageDBTypeTrait for IntType {

    fn is_inlined(&self) -> bool {
        true
    }

    fn get_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn len(&self) -> u32 {
        unimplemented!()
    }

    fn get_data_from_slice(_storage: &[u8]) -> &[u8] {
        unimplemented!()
    }
}
