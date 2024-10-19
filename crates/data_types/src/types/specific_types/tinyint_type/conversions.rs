use crate::{BigIntType, BigIntUnderlyingType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, TinyIntType, TinyIntUnderlyingType, Value, VarcharType};
use error_utils::anyhow::anyhow;

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

impl From<Option<TinyIntUnderlyingType>> for TinyIntType {
    fn from(value: Option<TinyIntUnderlyingType>) -> Self {
        TinyIntType::from(value.unwrap_or(TinyIntType::NULL))
    }
}

impl From<&Option<TinyIntUnderlyingType>> for TinyIntType {
    fn from(value: &Option<TinyIntUnderlyingType>) -> Self {
        TinyIntType::new(value.unwrap_or(TinyIntType::NULL))
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

impl From<&TinyIntType> for SmallIntType {
    fn from(v: &TinyIntType) -> SmallIntType {
        if v.is_null() {
            return SmallIntType::default().into();
        }

        SmallIntType::new(v.0 as SmallIntUnderlyingType).into()
    }
}

impl From<&TinyIntType> for IntType {
    fn from(v: &TinyIntType) -> IntType {
        if v.is_null() {
            return IntType::default().into();
        }

        IntType::new(v.0 as IntUnderlyingType).into()
    }
}

impl From<&TinyIntType> for BigIntType {
    fn from(v: &TinyIntType) -> BigIntType {
        if v.is_null() {
            return BigIntType::default().into();
        }

        BigIntType::new(v.0 as BigIntUnderlyingType).into()
    }
}

impl From<&TinyIntType> for DecimalType {
    fn from(v: &TinyIntType) -> DecimalType {
        if v.is_null() {
            return DecimalType::default();
        }

        DecimalType::new(v.0 as DecimalUnderlyingType)
    }
}

impl From<&TinyIntType> for VarcharType {
    fn from(v: &TinyIntType) -> VarcharType {
        if v.is_null() {
            return VarcharType::default();
        }

        VarcharType::from(v.0.to_string())
    }
}

impl ConversionDBTypeTrait for TinyIntType {
    fn as_string(&self) -> String {
        if self.is_null() {
            return "tinyint_null".to_string();
        }

        self.0.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE].copy_from_slice(self.0.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        TinyIntType::new(TinyIntUnderlyingType::from_ne_bytes(storage[..Self::SIZE].try_into().unwrap()))
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
