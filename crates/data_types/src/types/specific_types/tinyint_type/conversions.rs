use crate::{TinyIntUnderlyingType, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, StorageDBTypeTrait, TinyIntType, Value};
use anyhow::anyhow;

impl From<TinyIntUnderlyingType> for TinyIntType {
    fn from(value: TinyIntUnderlyingType) -> Self {
        TinyIntType::new(value)
    }
}

impl From<&TinyIntUnderlyingType> for TinyIntType {
    fn from(value: &TinyIntUnderlyingType) -> Self {
        TinyIntType::new(*value)
    }
}

impl From<&[u8]> for TinyIntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        TinyIntType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for TinyIntType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::TINYINT(self)
    }
}

impl Into<Value> for TinyIntType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl ConversionDBTypeTrait for TinyIntType {

    fn as_string(&self) -> String {
        if self.is_null() {
            return "tinyint_null".to_string();
        }

        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        TinyIntType::new(TinyIntUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                todo!()
            }
            DBTypeId::TINYINT => {
                Ok(self.clone().into())
            }
            DBTypeId::SMALLINT => {
                if self.is_null() {
                    return Ok(SmallIntType::default().into());
                }

                Ok(SmallIntType::new(self.value as SmallIntUnderlyingType).into())
            }
            DBTypeId::INT => {
                if self.is_null() {
                    return Ok(IntType::default().into());
                }

                Ok(IntType::new(self.value as IntUnderlyingType).into())
            }
            DBTypeId::BIGINT => {
                if self.is_null() {
                    return Ok(BigIntType::default().into());
                }

                Ok(BigIntType::new(self.value as BigIntUnderlyingType).into())
            }
            DBTypeId::DECIMAL => {
                if self.is_null() {
                    return Ok(DecimalType::default().into());
                }

                Ok(DecimalType::new(self.value as DecimalUnderlyingType).into())
            }
            DBTypeId::VARCHAR => {
                todo!()
            }
            DBTypeId::TIMESTAMP => {
                todo!()
            }
            _ => Err(anyhow!(format!("tinyint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
