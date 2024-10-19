use crate::types::StorageDBTypeTrait;
use crate::{VarcharType, VariableLengthStorageDBTypeTrait};

impl StorageDBTypeTrait for VarcharType {
    fn is_inlined(&self) -> bool {
        false
    }
}

impl VariableLengthStorageDBTypeTrait for VarcharType {
    fn get_data(&self) -> &[u8] {
        unimplemented!()
    }

    fn len(&self) -> u32 {
        self.len
    }

    // TODO - is this correct?
    fn get_data_from_slice(storage: &[u8]) -> &[u8] {
        storage
    }
}
