// TODO - should probably be trait

use crate::run_on_impl;
use crate::types::{ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl};


// TODO - implement from src/include/type/value.h
pub struct Value {
    /// The data type
    // type_id: TypeId,
    value: DBTypeIdImpl,
}


impl Value {
    pub fn new(value: DBTypeIdImpl) -> Self {
        Value {
            value
        }
    }

    #[inline]
    pub(crate) fn get_db_type_id(&self) -> DBTypeId {
        self.value.db_type_id()
    }

    pub(crate) fn get_value(&self) -> &DBTypeIdImpl {
        &self.value
    }

    pub fn try_cast_as(&self, db_type_id: DBTypeId) -> anyhow::Result<Value> {
        let new = run_on_impl!(&self.value, current, {
           current.try_cast_as(db_type_id)?
        });

        Ok(Value::new(new))
    }

    unsafe fn cast_as_unchecked(&self, db_type_id: DBTypeId) -> Value {
        self.try_cast_as(db_type_id).expect("cannot cast as the requested_type")
    }

    pub fn is_zero(&self) -> bool {
        run_on_impl!(&self.value, v, {
            v.is_zero()
        })
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


impl Default for Value {
    fn default() -> Self {
        // TODO - should use invalid type
        todo!()
    }
}

