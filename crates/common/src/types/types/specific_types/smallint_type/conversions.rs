use crate::types::{SmallIntType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, Value, ComparisonDBTypeTrait, TinyIntType, TinyIntUnderlyingType, IntType, IntUnderlyingType, BigIntType, BigIntUnderlyingType, DecimalType, DecimalUnderlyingType};
use anyhow::anyhow;
use crate::assert_in_range;
use super::SmallIntUnderlyingType;

impl From<SmallIntUnderlyingType> for SmallIntType {
    fn from(value: SmallIntUnderlyingType) -> Self {
        SmallIntType::new(value)
    }
}

impl From<&SmallIntUnderlyingType> for SmallIntType {
    fn from(value: &SmallIntUnderlyingType) -> Self {
        SmallIntType::new(*value)
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
        DBTypeIdImpl::SMALLINT(self)
    }
}

impl Into<Value> for SmallIntType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl ConversionDBTypeTrait for SmallIntType {

    fn as_string(&self) -> String {
        if self.is_null() {
            return "smallint_null".to_string();
        }

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
                if self.is_null() {
                    return Ok(TinyIntType::default().into());
                }

                assert_in_range!(TinyIntType, self.value, SmallIntUnderlyingType);
                Ok(TinyIntType::new(self.value as TinyIntUnderlyingType).into())
            }
            DBTypeId::SMALLINT => {
                Ok(self.clone().into())
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
            _ => Err(anyhow!(format!("smallint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
