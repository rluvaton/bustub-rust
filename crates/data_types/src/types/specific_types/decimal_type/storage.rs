use crate::types::{DecimalType, StorageDBTypeTrait};


impl Clone for DecimalType {
    fn clone(&self) -> Self {
        DecimalType::new(self.value)
    }
}

impl StorageDBTypeTrait for DecimalType {

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
