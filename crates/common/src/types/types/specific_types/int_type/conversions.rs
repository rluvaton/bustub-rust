use crate::types::{IntType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, Value, ComparisonDBTypeTrait, SmallIntType, SmallIntUnderlyingType, BigIntType, BigIntUnderlyingType, DecimalType, DecimalUnderlyingType, TinyIntType, TinyIntUnderlyingType};
use anyhow::anyhow;
use crate::assert_in_range;
use super::IntUnderlyingType;

impl From<IntUnderlyingType> for IntType {
    fn from(value: IntUnderlyingType) -> Self {
        IntType::new(value)
    }
}

impl From<&IntUnderlyingType> for IntType {
    fn from(value: &IntUnderlyingType) -> Self {
        IntType::new(*value)
    }
}

impl From<&[u8]> for IntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        IntType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for IntType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::INT(self)
    }
}

impl Into<Value> for IntType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl ConversionDBTypeTrait for IntType {

    fn as_string(&self) -> String {
        if self.is_null() {
            return "int_null".to_string();
        }

        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        IntType::new(IntUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                todo!()
            }
            DBTypeId::TINYINT => {
                if self.is_null() {
                    return Ok(TinyIntType::default().into());
                }

                assert_in_range!(TinyIntType, self.value, IntUnderlyingType);
                Ok(TinyIntType::new(self.value as TinyIntUnderlyingType).into())
            }
            DBTypeId::SMALLINT => {
                if self.is_null() {
                    return Ok(SmallIntType::default().into());
                }

                assert_in_range!(SmallIntType, self.value, IntUnderlyingType);
                Ok(SmallIntType::new(self.value as SmallIntUnderlyingType).into())
            }
            DBTypeId::INT => {
                Ok(self.clone().into())
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
            _ => Err(anyhow!(format!("int is not coercable to {}", db_type_id.get_name())))
        }
    }
}
