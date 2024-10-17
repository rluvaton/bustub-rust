use crate::{BooleanType, BooleanUnderlyingType, ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, Value, VarcharType};
use error_utils::anyhow::anyhow;

impl From<BooleanUnderlyingType> for BooleanType {
    fn from(value: BooleanUnderlyingType) -> Self {
        BooleanType::new(value)
    }
}

impl From<&BooleanUnderlyingType> for BooleanType {
    fn from(value: &BooleanUnderlyingType) -> Self {
        BooleanType::new(*value)
    }
}

impl From<bool> for BooleanType {
    fn from(value: bool) -> Self {
        BooleanType::new(if value { Self::TRUE} else {Self::FALSE})
    }
}

impl From<&bool> for BooleanType {
    fn from(value: &bool) -> Self {
        BooleanType::new(if *value { Self::TRUE} else {Self::FALSE})
    }
}

impl From<Option<bool>> for BooleanType {
    fn from(value: Option<bool>) -> Self {
        if let Some(value) = value {
            return value.into()
        }

        BooleanType::new(Self::NULL)
    }
}

impl From<&Option<bool>> for BooleanType {
    fn from(value: &Option<bool>) -> Self {
        if let Some(value) = value {
            return value.into()
        }

        BooleanType::new(Self::NULL)
    }
}

impl From<&[u8]> for BooleanType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        BooleanType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for BooleanType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::BOOLEAN(self)
    }
}

impl From<&BooleanType> for VarcharType {
    fn from(v: &BooleanType) -> VarcharType {
        if v.is_null() {
            return VarcharType::default()
        }

        VarcharType::from(v.as_string())
    }
}

impl Into<Value> for BooleanType {
    fn into(self) -> Value {
        Value::new(
            DBTypeIdImpl::BOOLEAN(
                self
            )
        )
    }
}

impl ConversionDBTypeTrait for BooleanType {

    fn as_string(&self) -> String {
        if self.value == Self::TRUE {
            "true".to_string()
        } else if self.value == Self::FALSE {
            "false".to_string()
        } else {
            // Null
            "boolean_null".to_string()
        }
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        BooleanType::new(BooleanUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::BOOLEAN => {
                Ok(self.clone().into())
            }
            DBTypeId::VARCHAR => {
                Ok(VarcharType::from(self).into())
            }
            _ => Err(anyhow!(format!("boolean is not coercable to {}", db_type_id.get_name())))
        }
    }
}
