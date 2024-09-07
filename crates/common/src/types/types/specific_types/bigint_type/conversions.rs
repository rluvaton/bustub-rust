use crate::types::{BigIntType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, StorageDBTypeTrait, TinyIntType, TinyIntUnderlyingType};
use anyhow::anyhow;
use crate::assert_in_range;
use super::BigIntUnderlyingType;

impl From<BigIntUnderlyingType> for BigIntType {
    fn from(value: BigIntUnderlyingType) -> Self {
        BigIntType::new(value)
    }
}

impl From<&BigIntUnderlyingType> for BigIntType {
    fn from(value: &BigIntUnderlyingType) -> Self {
        BigIntType::new(*value)
    }
}

impl From<&[u8]> for BigIntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        BigIntType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for BigIntType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::BIGINT(self)
    }
}

impl ConversionDBTypeTrait for BigIntType {

    fn to_string(&self) -> String {
        if self.is_null() {
            return "bigint_null".to_string();
        }

        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        BigIntType::new(BigIntUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
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

                assert_in_range!(TinyIntType, self.value, BigIntUnderlyingType);
                Ok(TinyIntType::new(self.value as TinyIntUnderlyingType).into())
            }
            DBTypeId::SMALLINT => {
                if self.is_null() {
                    return Ok(SmallIntType::default().into());
                }

                assert_in_range!(SmallIntType, self.value, BigIntUnderlyingType);
                Ok(SmallIntType::new(self.value as SmallIntUnderlyingType).into())
            }
            DBTypeId::INT => {
                if self.is_null() {
                    return Ok(IntType::default().into());
                }

                assert_in_range!(IntType, self.value, BigIntUnderlyingType);
                Ok(IntType::new(self.value as IntUnderlyingType).into())
            }
            DBTypeId::BIGINT => {
                Ok(self.clone().into())
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
            _ => Err(anyhow!(format!("bigint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
