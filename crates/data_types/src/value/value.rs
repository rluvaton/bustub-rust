// TODO - should probably be trait

use crate::types::{BigIntType, BooleanType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, IntType, SmallIntType, StorageDBTypeTrait, TimestampType, TinyIntType, VariableLengthStorageDBTypeTrait};
use crate::run_on_impl;
use std::fmt::{Display, Formatter};


// TODO - implement from src/include/type/value.h
#[derive(Debug)]
pub struct Value {
    /// The data type
    pub(super) value: DBTypeIdImpl,
}


impl Value {
    pub fn new(value: DBTypeIdImpl) -> Self {
        Value {
            value
        }
    }

    #[inline]
    pub fn get_db_type_id(&self) -> DBTypeId {
        self.value.db_type_id()
    }

    pub fn get_value(&self) -> &DBTypeIdImpl {
        &self.value
    }

    pub fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<Value> {
        let new = run_on_impl!(&self.value, current, {
           current.try_cast_as(db_type_id)?
        });

        Ok(Value::new(new))
    }

    #[allow(unused)]
    unsafe fn cast_as_unchecked(&self, db_type_id: DBTypeId) -> Value {
        self.try_cast_as(db_type_id).expect("cannot cast as the requested_type")
    }

    pub fn is_zero(&self) -> bool {
        run_on_impl!(&self.value, v, {
            v.is_zero()
        })
    }

    pub fn is_null(&self) -> bool {
        run_on_impl!(&self.value, v, {
            v.is_null()
        })
    }
    /// Deserialize a value of the given type from the given storage space.
    pub fn deserialize_from_slice(value_type: DBTypeId, slice: &[u8]) -> Self {
        let db_impl: DBTypeIdImpl = match value_type {
            DBTypeId::INVALID => unimplemented!(),
            DBTypeId::BOOLEAN => BooleanType::from(slice).into(),
            DBTypeId::TINYINT => TinyIntType::from(slice).into(),
            DBTypeId::SMALLINT => SmallIntType::from(slice).into(),
            DBTypeId::INT => IntType::from(slice).into(),
            DBTypeId::BIGINT => BigIntType::from(slice).into(),
            DBTypeId::DECIMAL => DecimalType::from(slice).into(),
            DBTypeId::VARCHAR => unimplemented!(),
            DBTypeId::TIMESTAMP => TimestampType::from(slice).into(),
        };

        Self::new(db_impl)
    }
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


    #[allow(unused)]
    fn to_string_value(&self) -> String {
        run_on_impl!(&self.value, v, {
            v.as_string()
        })
    }

    #[allow(unused)]
    pub fn serialize_to(&self, storage: &mut [u8]) {
        run_on_impl!(&self.value, v, {
            v.serialize_to(storage)
        })
    }


    #[allow(unused)]
    fn is_inlined(&self) -> bool {
        run_on_impl!(&self.value, v, {
            v.is_inlined()
        })
    }

    #[allow(unused)]
    fn get_data(&self) -> &[u8] {
        match &self.value {
            DBTypeIdImpl::VARCHAR(v) => v.get_data(),
            _ => unreachable!()
        }
    }

    #[allow(unused)]
    pub fn len(&self) -> u32 {
        match &self.value {
            DBTypeIdImpl::VARCHAR(v) => v.len() as u32,
            _ => unreachable!()
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        run_on_impl!(&self.value, v, {
            f.write_str(v.as_string().as_str())
        })
    }
}


impl Default for Value {
    fn default() -> Self {
        // TODO - should use invalid type
        todo!()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Value::new(
            run_on_impl!(&self.value, v, {
                v.clone().into()
            })
        )
    }
}
