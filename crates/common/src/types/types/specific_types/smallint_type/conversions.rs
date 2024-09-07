use crate::types::{SmallIntType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, Value};
use anyhow::anyhow;
use super::SmallIntUnderlyingType;

impl From<SmallIntUnderlyingType> for SmallIntType {
    fn from(value: SmallIntUnderlyingType) -> Self {
        SmallIntType::new(value)
    }
}

impl From<&[u8]> for SmallIntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        SmallIntType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for SmallIntType {
    fn into(self) -> DBTypeIdImpl {
        todo!()
        // DBTypeIdImpl::SMALLINT(self)
    }
}

impl Into<Value> for SmallIntType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl ConversionDBTypeTrait for SmallIntType {

    fn to_string(&self) -> String {
        // TODO - what about null
        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        SmallIntType::new(SmallIntUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                todo!()
            }
            DBTypeId::TINYINT => {
                todo!()
            }
            DBTypeId::SMALLINT => {
                Ok(self.clone().into())
            }
            DBTypeId::INTEGER => {
                todo!()
            }
            DBTypeId::BIGINT => {
                todo!()
            }
            DBTypeId::DECIMAL => {
                todo!()
            }
            DBTypeId::VARCHAR => {
                todo!()
            }
            DBTypeId::TIMESTAMP => {
                todo!()
            }
            _ => Err(anyhow!(format!("smallint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
