use crate::types::{ComparisonDBTypeTrait, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, DecimalType, DecimalUnderlyingType, IntType, IntUnderlyingType, SmallIntType, SmallIntUnderlyingType, StorageDBTypeTrait, TinyIntType, TinyIntUnderlyingType};
use crate::{assert_in_range, return_error_on_out_of_range, BigIntType, BigIntUnderlyingType, BooleanType, TimestampType, VarcharType, BUSTUB_VALUE_NULL};
use error_utils::anyhow::anyhow;
use error_utils::ToAnyhowResult;
use crate::types::errors::{InnerFromStringConversionError, NumericConversionError};

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

impl TryInto<DecimalType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<DecimalType, Self::Error> {
        if self.is_null() {
            return Ok(DecimalType::default().into());
        }

        self.value.parse::<DecimalUnderlyingType>()
            .map(|value| DecimalType::from(value))
            .map_err(|err| InnerFromStringConversionError::UnableToConvert {
                value: self.value.clone(),
                dest_type: DBTypeId::DECIMAL,
            })
    }
}

impl TryInto<BooleanType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<BooleanType, Self::Error> {
        if self.is_null() {
            return Ok(BooleanType::default().into());
        }

        if self.value == "true" || self.value == "TRUE" || self.value == "1" {
            Ok(BooleanType::from(true))
        } else if self.value == "false" || self.value == "FALSE" || self.value == "0" {
            Ok(BooleanType::from(false))
        } else {
            Err(InnerFromStringConversionError::UnableToConvert {
                value: self.value,
                dest_type: DBTypeId::BOOLEAN,
            })
        }
    }
}
impl TryInto<TimestampType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<TimestampType, Self::Error> {
        if self.is_null() {
            return Ok(TimestampType::default().into());
        }

        unimplemented!("Need to implement this")
    }
}

impl TryInto<TinyIntType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<TinyIntType, Self::Error> {
        let val: BigIntType = self.try_into()
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: self.value.clone(),
                    dest_type: DBTypeId::TINYINT,
                }
            })?;

        TryInto::<TinyIntType>::try_into(val)
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: self.value.clone(),
                    dest_type: DBTypeId::TINYINT,
                }
            })
    }
}

impl TryInto<SmallIntType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<SmallIntType, Self::Error> {
        let val: BigIntType = self.try_into()
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: self.value.clone(),
                    dest_type: DBTypeId::SMALLINT,
                }
            })?;

        TryInto::<SmallIntType>::try_into(val)
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: self.value.clone(),
                    dest_type: DBTypeId::SMALLINT,
                }
            })
    }
}

impl TryInto<IntType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<IntType, Self::Error> {
        let val: BigIntType = self.try_into()
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: self.value.clone(),
                    dest_type: DBTypeId::INT,
                }
            })?;

        TryInto::<IntType>::try_into(val)
            .map_err(|_| {
                InnerFromStringConversionError::UnableToConvert {
                    value: self.value.clone(),
                    dest_type: DBTypeId::INT,
                }
            })
    }
}

impl TryInto<BigIntType> for VarcharType {
    type Error = InnerFromStringConversionError;

    fn try_into(self) -> Result<BigIntType, Self::Error> {
        if self.is_null() {
            return Ok(BigIntType::default().into());
        }

        self.value.parse::<BigIntUnderlyingType>()
            .map(|value| BigIntType::from(value))
            .map_err(|err| InnerFromStringConversionError::UnableToConvert {
                value: self.value.clone(),
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
