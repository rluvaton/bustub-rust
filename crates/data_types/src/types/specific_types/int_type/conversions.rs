use crate::types::errors::NumericConversionError;
use crate::{return_error_on_out_of_range, BigIntType, BigIntUnderlyingType, BooleanType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TimestampType, TinyIntType, TinyIntUnderlyingType, Value, VarcharType};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;

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

impl From<Option<IntUnderlyingType>> for IntType {
    fn from(value: Option<IntUnderlyingType>) -> Self {
        IntType::from(value.unwrap_or(IntType::NULL))
    }
}

impl From<&Option<IntUnderlyingType>> for IntType {
    fn from(value: &Option<IntUnderlyingType>) -> Self {
        IntType::new(value.unwrap_or(IntType::NULL))
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

impl TryFrom<&IntType> for TinyIntType {
    type Error = NumericConversionError;

    fn try_from(v: &IntType) -> Result<TinyIntType, Self::Error> {
        if v.is_null() {
            return Ok(TinyIntType::default().into());
        }

        return_error_on_out_of_range!(TinyIntType, v.0, IntUnderlyingType);

        Ok(TinyIntType::new(v.0 as TinyIntUnderlyingType).into())
    }
}

impl TryFrom<&IntType> for SmallIntType {
    type Error = NumericConversionError;

    fn try_from(v: &IntType) -> Result<SmallIntType, Self::Error> {
        if v.is_null() {
            return Ok(SmallIntType::default().into());
        }

        return_error_on_out_of_range!(SmallIntType, v.0, IntUnderlyingType);

        Ok(SmallIntType::new(v.0 as SmallIntUnderlyingType).into())
    }
}

impl From<&IntType> for BigIntType {
    fn from(v: &IntType) -> BigIntType {
        if v.is_null() {
            return BigIntType::default().into();
        }

        BigIntType::new(v.0 as BigIntUnderlyingType).into()
    }
}

impl From<&IntType> for DecimalType {
    fn from(v: &IntType) -> DecimalType {
        if v.is_null() {
            return DecimalType::default()
        }

        DecimalType::new(v.0 as DecimalUnderlyingType)
    }
}

impl From<&IntType> for VarcharType {
    fn from(v: &IntType) -> VarcharType {
        if v.is_null() {
            return VarcharType::default()
        }

        VarcharType::from(v.0.to_string())
    }
}

impl Into<Value> for IntType {
    fn into(self) -> Value {
        Value::new(
            DBTypeIdImpl::INT(
                self
            )
        )
    }
}

impl ConversionDBTypeTrait for IntType {

    fn as_string(&self) -> String {
        if self.is_null() {
            return "int_null".to_string();
        }

        self.0.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE].copy_from_slice(self.0.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        IntType::new(IntUnderlyingType::from_ne_bytes(storage[..Self::SIZE].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                if self.is_null() {
                    Ok(BooleanType::from(None).into())
                } else {
                    Err(error_utils::anyhow!("Unable to cast non null Int to Boolean"))
                }
            }
            DBTypeId::TINYINT => {
                TinyIntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::SMALLINT => {
                SmallIntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::INT => {
                Ok(self.clone().into())
            }
            DBTypeId::BIGINT => {
                Ok(BigIntType::from(self).into())
            }
            DBTypeId::DECIMAL => {
                Ok(DecimalType::from(self).into())
            }
            DBTypeId::VARCHAR => {
                Ok(VarcharType::from(self).into())
            }
            DBTypeId::TIMESTAMP => {
                if self.is_null() {
                    Ok(TimestampType::from(None).into())
                } else {
                    Err(error_utils::anyhow!("Unable to cast non null Int to Timestamp"))
                }
            }
            _ => Err(anyhow!(format!("int is not coercable to {}", db_type_id.get_name())))
        }
    }
}
