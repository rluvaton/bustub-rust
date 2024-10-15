use crate::types::{TinyIntType, StorageDBTypeTrait};
use super::TinyIntUnderlyingType;


impl Clone for TinyIntType {
    fn clone(&self) -> Self {
        TinyIntType::new(self.value)
    }
}

impl StorageDBTypeTrait for TinyIntType {
    const SIZE: usize = size_of::<TinyIntUnderlyingType>();

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
