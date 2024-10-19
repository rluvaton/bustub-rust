use crate::types::errors::NumericConversionError;
use crate::{return_error_on_out_of_range, BigIntType, BigIntUnderlyingType, BooleanType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TimestampType, TinyIntType, TinyIntUnderlyingType, Value, VarcharType};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;

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

impl From<Option<SmallIntUnderlyingType>> for SmallIntType {
    fn from(value: Option<SmallIntUnderlyingType>) -> Self {
        SmallIntType::from(value.unwrap_or(SmallIntType::NULL))
    }
}

impl From<&Option<SmallIntUnderlyingType>> for SmallIntType {
    fn from(value: &Option<SmallIntUnderlyingType>) -> Self {
        SmallIntType::new(value.unwrap_or(SmallIntType::NULL))
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

impl TryFrom<&SmallIntType> for TinyIntType {
    type Error = NumericConversionError;

    fn try_from(v: &SmallIntType) -> Result<TinyIntType, Self::Error> {
        if v.is_null() {
            return Ok(TinyIntType::default().into());
        }

        return_error_on_out_of_range!(TinyIntType, v.0, SmallIntUnderlyingType);

        Ok(TinyIntType::new(v.0 as TinyIntUnderlyingType).into())
    }
}

impl From<&SmallIntType> for IntType {
    fn from(v: &SmallIntType) -> IntType {
        if v.is_null() {
            return IntType::default();
        }

        IntType::new(v.0 as IntUnderlyingType)
    }
}

impl From<&SmallIntType> for BigIntType {
    fn from(v: &SmallIntType) -> BigIntType {
        if v.is_null() {
            return BigIntType::default();
        }

        BigIntType::new(v.0 as BigIntUnderlyingType)
    }
}

impl From<&SmallIntType> for DecimalType {
    fn from(v: &SmallIntType) -> DecimalType {
        if v.is_null() {
            return DecimalType::default()
        }

        DecimalType::new(v.0 as DecimalUnderlyingType)
    }
}

impl From<&SmallIntType> for VarcharType {
    fn from(v: &SmallIntType) -> VarcharType {
        if v.is_null() {
            return VarcharType::default()
        }

        VarcharType::from(v.0.to_string())
    }
}

impl ConversionDBTypeTrait for SmallIntType {
    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE].copy_from_slice(self.0.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        SmallIntType::new(SmallIntUnderlyingType::from_ne_bytes(storage[..Self::SIZE].try_into().unwrap()))
    }

    fn as_string(&self) -> String {
        if self.is_null() {
            return "smallint_null".to_string();
        }

        self.0.to_string()
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                if self.is_null() {
                    Ok(BooleanType::from(None).into())
                } else {
                    Err(error_utils::anyhow!("Unable to cast non null SmallInt to Boolean"))
                }
            }
            DBTypeId::TINYINT => {
                TinyIntType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::SMALLINT => {
                Ok(self.clone().into())
            }
            DBTypeId::INT => {
                Ok(IntType::from(self).into())
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
                    Err(error_utils::anyhow!("Unable to cast non null SmallInt to Timestamp"))
                }
            }
            _ => Err(anyhow!(format!("smallint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
