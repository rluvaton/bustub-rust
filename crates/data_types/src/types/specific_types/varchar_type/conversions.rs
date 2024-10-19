use crate::types::errors::InnerFromStringConversionError;
use crate::types::{ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, SmallIntType, TinyIntType};
use crate::{BigIntType, BigIntUnderlyingType, BooleanType, TimestampType, Value, VarcharType, BUSTUB_VALUE_NULL};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;

impl From<&str> for VarcharType {
    fn from(value: &str) -> Self {
        Self::assert_valid_before_creation(value);
        VarcharType {
            len: value.len() as u32,
            value: value.to_string(),
        }
    }
}

impl From<Option<&str>> for VarcharType {
    fn from(value: Option<&str>) -> Self {
        Self::new(value)
    }
}

impl From<String> for VarcharType {
    fn from(value: String) -> Self {
        Self::assert_valid_before_creation(value.as_str());
        VarcharType {
            len: value.len() as u32,
            value,
        }
    }
}

impl From<Option<String>> for VarcharType {
    fn from(value: Option<String>) -> Self {
        if let Some(value) = value {
            Self::assert_valid_before_creation(value.as_str());
            VarcharType {
                len: value.len() as u32,
                value,
            }
        } else {
            VarcharType {
                len: BUSTUB_VALUE_NULL,
                value: "".to_string(),
            }
        }
    }
}


impl From<&[u8]> for VarcharType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        VarcharType::deserialize_from(value)
    }
}


impl Into<DBTypeIdImpl> for VarcharType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::VARCHAR(self)
    }
}

impl Into<Value> for VarcharType {
    fn into(self) -> Value {
        Value::new(
            DBTypeIdImpl::VARCHAR(
                self
            )
        )
    }
}

impl From<&Value> for VarcharType {
    fn from(value: &Value) -> VarcharType {
        match value.get_value() {
            DBTypeIdImpl::BOOLEAN(v) => v.into(),
            DBTypeIdImpl::TINYINT(v) => v.into(),
            DBTypeIdImpl::SMALLINT(v) => v.into(),
            DBTypeIdImpl::INT(v) => v.into(),
            DBTypeIdImpl::BIGINT(v) => v.into(),
            DBTypeIdImpl::DECIMAL(v) => v.into(),
            DBTypeIdImpl::VARCHAR(v) => v.into(),
            DBTypeIdImpl::TIMESTAMP(v) => v.into(),
        }
    }
}

impl From<&VarcharType> for VarcharType {
    fn from(value: &VarcharType) -> Self {
        value.clone()
    }
}

impl TryFrom<&VarcharType> for DecimalType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<DecimalType, Self::Error> {
        if v.is_null() {
            return Ok(DecimalType::default().into());
        }

        v.value.parse::<DecimalUnderlyingType>()
            .map(|value| DecimalType::from(value))
            .map_err(|_| InnerFromStringConversionError::UnableToConvert {
                value: v.value.clone(),
                dest_type: DBTypeId::DECIMAL,
            })
    }
}

impl TryFrom<&VarcharType> for BooleanType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<BooleanType, Self::Error> {
        if v.is_null() {
            return Ok(BooleanType::default().into());
        }

        if v.value == "true" || v.value == "TRUE" || v.value == "1" {
            Ok(BooleanType::from(true))
        } else if v.value == "false" || v.value == "FALSE" || v.value == "0" {
            Ok(BooleanType::from(false))
        } else {
            Err(InnerFromStringConversionError::UnableToConvert {
                value: v.value.clone(),
                dest_type: DBTypeId::BOOLEAN,
            })
        }
    }
}
impl TryFrom<&VarcharType> for TimestampType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<TimestampType, Self::Error> {
        if v.is_null() {
            return Ok(TimestampType::default().into());
        }

        unimplemented!("Need to implement this")
    }
}

impl TryFrom<&VarcharType> for TinyIntType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<TinyIntType, Self::Error> {
        let val: Result<BigIntType, Self::Error> = BigIntType::try_from(v);

        let val = match &val {
            Ok(v) => v,
            Err(err) => {
                return Err(match err {
                    InnerFromStringConversionError::UnableToConvert { value, .. } => {
                        InnerFromStringConversionError::UnableToConvert {
                            value: value.clone(),
                            dest_type: DBTypeId::INT
                        }
                    }
                })
            }
        };

        TinyIntType::try_from(val)
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: val.value.to_string(),
                    dest_type: DBTypeId::TINYINT,
                }
            })
    }
}

impl TryFrom<&VarcharType> for SmallIntType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<SmallIntType, Self::Error> {
        let val: Result<BigIntType, Self::Error> = BigIntType::try_from(v);

        let val = match &val {
            Ok(v) => v,
            Err(err) => {
                return Err(match err {
                    InnerFromStringConversionError::UnableToConvert { value, .. } => {
                        InnerFromStringConversionError::UnableToConvert {
                            value: value.clone(),
                            dest_type: DBTypeId::INT
                        }
                    }
                })
            }
        };

        SmallIntType::try_from(val)
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: v.value.clone(),
                    dest_type: DBTypeId::SMALLINT,
                }
            })
    }
}

impl TryFrom<&VarcharType> for IntType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<IntType, Self::Error> {
        let val: Result<BigIntType, Self::Error> = BigIntType::try_from(v);

        let val = match &val {
            Ok(v) => v,
            Err(err) => {
                return Err(match err {
                    InnerFromStringConversionError::UnableToConvert { value, .. } => {
                        InnerFromStringConversionError::UnableToConvert {
                            value: value.clone(),
                            dest_type: DBTypeId::INT
                        }
                    }
                })
            }
        };

        IntType::try_from(val)
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: v.value.clone(),
                    dest_type: DBTypeId::INT,
                }
            })
    }
}

impl TryFrom<&VarcharType> for BigIntType {
    type Error = InnerFromStringConversionError;

    fn try_from(v: &VarcharType) -> Result<BigIntType, Self::Error> {
        if v.is_null() {
            return Ok(BigIntType::default().into());
        }

        v.value.parse::<BigIntUnderlyingType>()
            .map(|value| BigIntType::from(value))
            .map_err(|_| InnerFromStringConversionError::UnableToConvert {
                value: v.value.clone(),
                dest_type: DBTypeId::BIGINT,
            })
    }
}

impl ConversionDBTypeTrait for VarcharType {
    fn as_string(&self) -> String {
        if self.is_null() {
            return "varlen_null".to_string();
        }

        self.value.clone()
    }

    // TODO - add tests for serializing null

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..size_of::<u32>()].copy_from_slice(self.len.to_ne_bytes().as_slice());

        // null is only checked by the length so no need to add the rest of the data
        if self.is_null() {
            return;
        }


        storage[size_of::<u32>()..size_of::<u32>() + self.len as usize].copy_from_slice(&self.value[..self.len as usize].as_bytes())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        let len = u32::from_ne_bytes(storage[..size_of::<u32>()].try_into().unwrap());
        if len == BUSTUB_VALUE_NULL {
            Self::default()
        } else {
            let str_bytes = &storage[size_of::<u32>()..size_of::<u32>() + len as usize];
            // Only utf8 is supported at the moment
            let str = std::str::from_utf8(str_bytes).expect("Must be able to parse UTF-8 string from storage");
            Self::from(str)
        }
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {
        match db_type_id {
            DBTypeId::BOOLEAN => {
                BooleanType::try_from(self).to_anyhow().map(|v| v.into())
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
                DecimalType::try_from(self).to_anyhow().map(|v| v.into())
            }
            DBTypeId::VARCHAR => {
                Ok(self.clone().into())
            }
            DBTypeId::TIMESTAMP => {
                TimestampType::try_from(self).to_anyhow().map(|v| v.into())
            }
            _ => Err(anyhow!(format!("bigint is not coercable to {}", db_type_id.get_name())))
        }
    }
}
