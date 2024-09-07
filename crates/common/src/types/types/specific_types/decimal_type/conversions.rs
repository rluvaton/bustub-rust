use crate::types::{IntType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, Value, DecimalType, ComparisonDBTypeTrait, TinyIntType, TinyIntUnderlyingType, SmallIntType, SmallIntUnderlyingType, BigIntType, BigIntUnderlyingType, IntUnderlyingType};
use anyhow::anyhow;
use crate::assert_in_range;
use super::DecimalUnderlyingType;

impl From<DecimalUnderlyingType> for DecimalType {
    fn from(value: DecimalUnderlyingType) -> Self {
        DecimalType::new(value)
    }
}

impl From<&DecimalUnderlyingType> for DecimalType {
    fn from(value: &DecimalUnderlyingType) -> Self {
        DecimalType::new(*value)
    }
}

impl From<&[u8]> for DecimalType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        DecimalType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for DecimalType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::DECIMAL(self)
    }
}

impl Into<Value> for DecimalType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl ConversionDBTypeTrait for DecimalType {

    fn to_string(&self) -> String {
        if self.is_null() {
            return "decimal_null".to_string();
        }

        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        DecimalType::new(DecimalUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
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

                assert_in_range!(TinyIntType, self.value, DecimalUnderlyingType);
                Ok(TinyIntType::new(self.value as TinyIntUnderlyingType).into())
            }
            DBTypeId::SMALLINT => {
                if self.is_null() {
                    return Ok(SmallIntType::default().into());
                }

                assert_in_range!(SmallIntType, self.value, DecimalUnderlyingType);
                Ok(SmallIntType::new(self.value as SmallIntUnderlyingType).into())
            }
            DBTypeId::INT => {
                if self.is_null() {
                    return Ok(IntType::default().into());
                }

                assert_in_range!(IntType, self.value, DecimalUnderlyingType);
                Ok(IntType::new(self.value as IntUnderlyingType).into())
            }
            DBTypeId::BIGINT => {
                if self.is_null() {
                    return Ok(BigIntType::default().into());
                }

                assert_in_range!(BigIntType, self.value, DecimalUnderlyingType);
                Ok(BigIntType::new(self.value as BigIntUnderlyingType).into())
            }
            DBTypeId::DECIMAL => {
                Ok(self.clone().into())
            }
            DBTypeId::VARCHAR => {
                todo!()
            }
            DBTypeId::TIMESTAMP => {
                todo!()
            }
            _ => Err(anyhow!(format!("decimal is not coercable to {}", db_type_id.get_name())))
        }
    }
}
