use crate::{assert_in_range, DecimalUnderlyingType, IntType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, Value, DecimalType, ComparisonDBTypeTrait, TinyIntType, TinyIntUnderlyingType, SmallIntType, SmallIntUnderlyingType, BigIntType, BigIntUnderlyingType, IntUnderlyingType, VarcharType, return_error_on_out_of_range};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;
use crate::types::errors::NumericConversionError;

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

impl From<&DecimalType> for VarcharType {
    fn from(v: &DecimalType) -> VarcharType {
        if v.is_null() {
            return VarcharType::default()
        }

        VarcharType::from(v.value.to_string())
    }
}

impl TryFrom<&DecimalType> for TinyIntType {
    type Error = NumericConversionError;

    fn try_from(v: &DecimalType) -> Result<TinyIntType, Self::Error> {
        if v.is_null() {
            return Ok(TinyIntType::default().into());
        }

        return_error_on_out_of_range!(TinyIntType, v.value, DecimalUnderlyingType);

        Ok(TinyIntType::new(v.value as TinyIntUnderlyingType).into())
    }
}

impl TryFrom<&DecimalType> for SmallIntType {
    type Error = NumericConversionError;

    fn try_from(v: &DecimalType) -> Result<SmallIntType, Self::Error> {
        if v.is_null() {
            return Ok(SmallIntType::default().into());
        }

        return_error_on_out_of_range!(SmallIntType, v.value, DecimalUnderlyingType);

        Ok(SmallIntType::new(v.value as SmallIntUnderlyingType).into())
    }
}

impl TryFrom<&DecimalType> for IntType {
    type Error = NumericConversionError;

    fn try_from(v: &DecimalType) -> Result<IntType, Self::Error> {
        if v.is_null() {
            return Ok(IntType::default().into());
        }

        return_error_on_out_of_range!(IntType, v.value, DecimalUnderlyingType);

        Ok(IntType::new(v.value as IntUnderlyingType).into())
    }
}

impl TryFrom<&DecimalType> for BigIntType {
    type Error = NumericConversionError;

    fn try_from(v: &DecimalType) -> Result<BigIntType, Self::Error> {
        if v.is_null() {
            return Ok(BigIntType::default().into());
        }

        return_error_on_out_of_range!(BigIntType, v.value, DecimalUnderlyingType);

        Ok(BigIntType::new(v.value as BigIntUnderlyingType).into())
    }
}

impl ConversionDBTypeTrait for DecimalType {

    fn as_string(&self) -> String {
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
                BigIntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::DECIMAL => {
                Ok(self.clone().into())
            }
            DBTypeId::VARCHAR => {
                Ok(VarcharType::from(self).into())
            }
            DBTypeId::TIMESTAMP => {
                todo!()
            }
            _ => Err(anyhow!(format!("decimal is not coercable to {}", db_type_id.get_name())))
        }
    }
}
