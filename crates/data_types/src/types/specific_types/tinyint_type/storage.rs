use crate::types::{StorageDBTypeTrait, TinyIntType};


impl Clone for TinyIntType {
    fn clone(&self) -> Self {
        TinyIntType::new(self.value)
    }
}

impl StorageDBTypeTrait for TinyIntType {

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
