use crate::types::{BooleanType, BooleanUnderlyingType, StorageDBTypeTrait};


impl Clone for BooleanType {
    fn clone(&self) -> Self {
        BooleanType::new(self.value)
    }
}

impl StorageDBTypeTrait for BooleanType {
    const SIZE: usize = size_of::<BooleanUnderlyingType>();

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
