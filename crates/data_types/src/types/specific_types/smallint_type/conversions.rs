use crate::types::errors::InnerNumericConversionError;
use crate::{SmallIntUnderlyingType, SmallIntType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, assert_in_range, ComparisonDBTypeTrait, TinyIntType, TinyIntUnderlyingType, IntType, IntUnderlyingType, BigIntType, BigIntUnderlyingType, DecimalType, DecimalUnderlyingType, Value, VarcharType, return_error_on_out_of_range};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;
use crate::types::errors::NumericConversionError;

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

impl TryInto<TinyIntType> for SmallIntType {
    type Error = NumericConversionError;

    fn try_into(self) -> Result<TinyIntType, Self::Error> {
        if self.is_null() {
            return Ok(TinyIntType::default().into());
        }

        return_error_on_out_of_range!(TinyIntType, self.value, SmallIntUnderlyingType);

        Ok(TinyIntType::new(self.value as TinyIntUnderlyingType).into())
    }
}

impl Into<IntType> for SmallIntType {
    fn into(self) -> IntType {
        if self.is_null() {
            return IntType::default().into();
        }

        IntType::new(self.value as IntUnderlyingType).into()
    }
}

impl Into<BigIntType> for SmallIntType {
    fn into(self) -> BigIntType {
        if self.is_null() {
            return BigIntType::default().into();
        }

        BigIntType::new(self.value as BigIntUnderlyingType).into()
    }
}

impl Into<DecimalType> for SmallIntType {
    fn into(self) -> DecimalType {
        if self.is_null() {
            return DecimalType::default()
        }

        DecimalType::new(self.value as DecimalUnderlyingType)
    }
}

impl Into<VarcharType> for SmallIntType {
    fn into(self) -> VarcharType {
        if self.is_null() {
            return VarcharType::default()
        }

        VarcharType::from(self.value.to_string())
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
                todo!()
            }
            _ => Err(anyhow!(format!("smallint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
