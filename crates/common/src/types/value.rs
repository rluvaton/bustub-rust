// TODO - should probably be trait

use crate::types::{CmpBool, CustomType, TypeId};


// TODO - implement from src/include/type/value.h
pub struct Value {
    /// The data type
    type_id: TypeId
}


impl Value {
    // TODO - this is deserialize_from
    pub fn deserialize_from_ptr(ptr: *const u8, value_type: TypeId) -> Self {
        todo!()
    }

    // TODO - this is deserialize_from
    pub fn deserialize_from_slice(slice: &[u8], value_type: TypeId) -> Self {
        todo!()
    }

    pub fn compare_equals(&self, o: &Value) -> CmpBool {
        CustomType::get_instance(self.type_id).compare_equals(&self, o)
    }

    pub fn compare_not_equals(&self, o: &Value) -> CmpBool {
        CustomType::get_instance(self.type_id).compare_not_equals(&self, o)
    }

    pub fn compare_less_than(&self, o: &Value) -> CmpBool {
        CustomType::get_instance(self.type_id).compare_less_than(&self, o)
    }

    pub fn compare_less_than_equals(&self, o: &Value) -> CmpBool {
        CustomType::get_instance(self.type_id).compare_less_than_equals(&self, o)
    }

    pub fn compare_greater_than(&self, o: &Value) -> CmpBool {
        CustomType::get_instance(self.type_id).compare_greater_than(&self, o)
    }

    pub fn compare_greater_than_equals(&self, o: &Value) -> CmpBool {
        CustomType::get_instance(self.type_id).compare_greater_than_equals(&self, o)
    }
}
