use super::BigIntUnderlyingType;
use crate::types::errors::NumericConversionError;
use crate::types::{BigIntType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TinyIntType, TinyIntUnderlyingType};
use crate::{return_error_on_out_of_range, Value, VarcharType};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;

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

impl From<Option<BigIntUnderlyingType>> for BigIntType {
    fn from(value: Option<BigIntUnderlyingType>) -> Self {
        BigIntType::from(value.unwrap_or(BigIntType::NULL))
    }
}

impl From<&Option<BigIntUnderlyingType>> for BigIntType {
    fn from(value: &Option<BigIntUnderlyingType>) -> Self {
        BigIntType::new(value.unwrap_or(BigIntType::NULL))
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

impl TryFrom<&BigIntType> for TinyIntType {
    type Error = NumericConversionError;

    fn try_from(value: &BigIntType) -> Result<TinyIntType, Self::Error> {
        if value.is_null() {
            return Ok(TinyIntType::default().into());
        }

        return_error_on_out_of_range!(TinyIntType, value.value, BigIntUnderlyingType);

        Ok(TinyIntType::new(value.value as TinyIntUnderlyingType).into())
    }
}

impl TryFrom<&BigIntType> for SmallIntType {
    type Error = NumericConversionError;

    fn try_from(value: &BigIntType) -> Result<SmallIntType, Self::Error> {
        if value.is_null() {
            return Ok(SmallIntType::default().into());
        }

        return_error_on_out_of_range!(SmallIntType, value.value, BigIntUnderlyingType);

        Ok(SmallIntType::new(value.value as SmallIntUnderlyingType).into())
    }
}

impl TryFrom<&BigIntType> for IntType {
    type Error = NumericConversionError;

    fn try_from(value: &BigIntType) -> Result<IntType, Self::Error> {
        if value.is_null() {
            return Ok(IntType::default().into());
        }

        return_error_on_out_of_range!(IntType, value.value, BigIntUnderlyingType);

        Ok(IntType::new(value.value as IntUnderlyingType).into())
    }
}

impl From<&BigIntType> for DecimalType {
    fn from(value: &BigIntType) -> DecimalType {
        if value.is_null() {
            return DecimalType::default();
        }

        DecimalType::new(value.value as DecimalUnderlyingType)
    }
}

impl From<&BigIntType> for VarcharType {
    fn from(value: &BigIntType) -> VarcharType {
        if value.is_null() {
            return VarcharType::default();
        }

        VarcharType::from(value.value.to_string())
    }
}

impl Into<Value> for BigIntType {
    fn into(self) -> Value {
        Value::new(
            DBTypeIdImpl::BIGINT(
                self
            )
        )
    }
}

impl ConversionDBTypeTrait for BigIntType {
    fn as_string(&self) -> String {
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

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                todo!()
            }
            DBTypeId::TINYINT => {
                TinyIntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::SMALLINT => {
                SmallIntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::INT => {
                IntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::BIGINT => {
                Ok(self.clone().into())
            }
            DBTypeId::DECIMAL => {
                Ok(DecimalType::from(self).into())
            }
            DBTypeId::VARCHAR => {
                Ok(VarcharType::from(self).into())
            }
            DBTypeId::TIMESTAMP => {
                todo!()
            }
            _ => Err(anyhow!(format!("bigint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
