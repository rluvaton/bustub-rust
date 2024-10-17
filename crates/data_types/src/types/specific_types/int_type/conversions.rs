use crate::{assert_in_range, return_error_on_out_of_range, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, StorageDBTypeTrait, TinyIntType, TinyIntUnderlyingType, Value, VarcharType};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;
use crate::types::errors::NumericConversionError;

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
        if let Some(value) = value {
            return value.into()
        }

        IntType::new(Self::NULL)
    }
}

impl From<&Option<IntUnderlyingType>> for IntType {
    fn from(value: &Option<IntUnderlyingType>) -> Self {
        if let Some(value) = value {
            return value.into()
        }

        IntType::new(Self::NULL)
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

impl TryFrom<&IntType> for TinyIntType {
    type Error = NumericConversionError;

    fn try_from(v: &IntType) -> Result<TinyIntType, Self::Error> {
        if v.is_null() {
            return Ok(TinyIntType::default().into());
        }

        return_error_on_out_of_range!(TinyIntType, v.value, IntUnderlyingType);

        Ok(TinyIntType::new(v.value as TinyIntUnderlyingType).into())
    }
}

impl TryFrom<&IntType> for SmallIntType {
    type Error = NumericConversionError;

    fn try_from(v: &IntType) -> Result<SmallIntType, Self::Error> {
        if v.is_null() {
            return Ok(SmallIntType::default().into());
        }

        return_error_on_out_of_range!(SmallIntType, v.value, IntUnderlyingType);

        Ok(SmallIntType::new(v.value as SmallIntUnderlyingType).into())
    }
}

impl From<&IntType> for BigIntType {
    fn from(v: &IntType) -> BigIntType {
        if v.is_null() {
            return BigIntType::default().into();
        }

        BigIntType::new(v.value as BigIntUnderlyingType).into()
    }
}

impl From<&IntType> for DecimalType {
    fn from(v: &IntType) -> DecimalType {
        if v.is_null() {
            return DecimalType::default()
        }

        DecimalType::new(v.value as DecimalUnderlyingType)
    }
}

impl From<&IntType> for VarcharType {
    fn from(v: &IntType) -> VarcharType {
        if v.is_null() {
            return VarcharType::default()
        }

        VarcharType::from(v.value.to_string())
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
                todo!()
            }
            _ => Err(anyhow!(format!("int is not coercable to {}", db_type_id.get_name())))
        }
    }
}
