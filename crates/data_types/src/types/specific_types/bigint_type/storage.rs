use crate::types::{BigIntType, StorageDBTypeTrait};

impl Clone for BigIntType {
    fn clone(&self) -> Self {
        BigIntType::new(self.0)
    }
}

impl StorageDBTypeTrait for BigIntType {

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
