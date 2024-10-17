use crate::{TinyIntUnderlyingType, BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, StorageDBTypeTrait, TinyIntType, Value, VarcharType, return_error_on_out_of_range};
use error_utils::anyhow::anyhow;
use crate::types::errors::NumericConversionError;

impl From<TinyIntUnderlyingType> for TinyIntType {
    fn from(value: TinyIntUnderlyingType) -> Self {
        TinyIntType::new(value)
    }
}

impl From<&TinyIntUnderlyingType> for TinyIntType {
    fn from(value: &TinyIntUnderlyingType) -> Self {
        TinyIntType::new(*value)
    }
}

impl From<&[u8]> for TinyIntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        TinyIntType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for TinyIntType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::TINYINT(self)
    }
}

impl Into<Value> for TinyIntType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl Into<SmallIntType> for TinyIntType {
    fn into(self) -> SmallIntType {
        if self.is_null() {
            return SmallIntType::default().into();
        }

        SmallIntType::new(self.value as SmallIntUnderlyingType).into()
    }
}

impl Into<IntType> for TinyIntType {
    fn into(self) -> IntType {
        if self.is_null() {
            return IntType::default().into();
        }

        IntType::new(self.value as IntUnderlyingType).into()
    }
}

impl Into<BigIntType> for TinyIntType {
    fn into(self) -> BigIntType {
        if self.is_null() {
            return BigIntType::default().into();
        }

        BigIntType::new(self.value as BigIntUnderlyingType).into()
    }
}

impl Into<DecimalType> for TinyIntType {
    fn into(self) -> DecimalType {
        if self.is_null() {
            return DecimalType::default();
        }

        DecimalType::new(self.value as DecimalUnderlyingType)
    }
}

impl Into<VarcharType> for TinyIntType {
    fn into(self) -> VarcharType {
        if self.is_null() {
            return VarcharType::default();
        }

        VarcharType::from(self.value.to_string())
    }
}

impl ConversionDBTypeTrait for TinyIntType {
    fn as_string(&self) -> String {
        if self.is_null() {
            return "tinyint_null".to_string();
        }

        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        TinyIntType::new(TinyIntUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                todo!()
            }
            DBTypeId::TINYINT => {
                Ok(self.clone().into())
            }
            DBTypeId::SMALLINT => {
                Ok(SmallIntType::from(self).into())
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
            _ => Err(anyhow!(format!("tinyint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
