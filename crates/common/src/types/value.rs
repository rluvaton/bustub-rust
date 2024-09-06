// TODO - should probably be trait

use crate::types::TypeId;


// TODO - implement from src/include/type/value.h
pub struct Value {
    /// The data type
    type_id: TypeId
}

impl Value {

    #[inline]
    fn get_type_id(&self) -> TypeId {
        self.type_id
    }

    // // TODO - this is deserialize_from
    // pub fn deserialize_from_ptr(ptr: *const u8, value_type: TypeId) -> Self {
    //     todo!()
    // }
    //
    // // TODO - this is deserialize_from
    // /// Deserialize a value of the given type from the given storage space.
    // pub fn deserialize_from_slice(slice: &[u8], value_type: TypeId) -> Self {
    //     value_type.deserialize_from(slice)
    // }
    //
    // pub fn compare_equals(&self, o: &Value) -> CmpBool {
    //     CustomType::get_instance(self.type_id).compare_equals(&self, o)
    // }
    //
    // pub fn compare_not_equals(&self, o: &Value) -> CmpBool {
    //     CustomType::get_instance(self.type_id).compare_not_equals(&self, o)
    // }
    //
    // pub fn compare_less_than(&self, o: &Value) -> CmpBool {
    //     CustomType::get_instance(self.type_id).compare_less_than(&self, o)
    // }
    //
    // pub fn compare_less_than_equals(&self, o: &Value) -> CmpBool {
    //     CustomType::get_instance(self.type_id).compare_less_than_equals(&self, o)
    // }
    //
    // pub fn compare_greater_than(&self, o: &Value) -> CmpBool {
    //     CustomType::get_instance(self.type_id).compare_greater_than(&self, o)
    // }
    //
    // pub fn compare_greater_than_equals(&self, o: &Value) -> CmpBool {
    //     CustomType::get_instance(self.type_id).compare_greater_than_equals(&self, o)
    // }
}

