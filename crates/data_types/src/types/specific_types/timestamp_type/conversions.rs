use crate::{TimestampType, TimestampUnderlyingType, ConversionDBTypeTrait, DBTypeId, DBTypeIdImpl, StorageDBTypeTrait, Value, ComparisonDBTypeTrait, VarcharType, TinyIntType};
use error_utils::anyhow::anyhow;

impl From<TimestampUnderlyingType> for TimestampType {
    fn from(value: TimestampUnderlyingType) -> Self {
        TimestampType::new(value)
    }
}

impl From<&TimestampUnderlyingType> for TimestampType {
    fn from(value: &TimestampUnderlyingType) -> Self {
        TimestampType::new(*value)
    }
}


impl From<&[u8]> for TimestampType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        TimestampType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for TimestampType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::TIMESTAMP(self)
    }
}

impl Into<Value> for TimestampType {
    fn into(self) -> Value {
        Value::new(self.into())
    }
}

impl From<&TimestampType> for VarcharType {
    fn from(value: &TimestampType) -> VarcharType {
        if value.is_null() {
            return VarcharType::default();
        }

        VarcharType::from(value.as_string())
    }
}

impl ConversionDBTypeTrait for TimestampType {

    fn as_string(&self) -> String {
        if self.is_null() {
            return "timestamp_null".to_string()
        }

        let mut tm = self.value;

        let micro = (tm % 1_000_000) as u32;
        tm /= 1_000_000;

        let mut second = (tm % 100_000) as u32;
        let sec = (second % 60) as u16;
        second /= 60;

        let min = (second % 60) as u16;
        second /= 60;

        let hour = (second % 24) as u16;
        tm /= 100_000;

        let year = (tm % 10_000) as u16;
        tm /= 10_000;

        let mut tz = (tm % 27) as i32;
        tz -= 12;
        tm /= 27;

        let day = (tm % 32) as u16;
        tm /= 32;

        let month = tm as u16;

        // Format the date and time part
        let date_str = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
            year, month, day, hour, min, sec, micro
        );

        // Format the timezone part
        let zone_sign = if tz >= 0 { '+' } else { '-' };
        let tz = tz.abs();
        let zone_str = format!("{:02}", tz);

        // Combine everything
        let mut result = date_str;
        result.push(zone_sign);
        result.push_str(&zone_str);

        result
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        TimestampType::new(TimestampUnderlyingType::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }

    fn try_cast_as(&self, db_type_id: DBTypeId) -> error_utils::anyhow::Result<DBTypeIdImpl> {

        // TODO - if null
        match db_type_id {
            DBTypeId::TIMESTAMP => {
                Ok(self.clone().into())
            }
            DBTypeId::VARCHAR => {
                Ok(VarcharType::from(self).into())
            }
            _ => Err(anyhow!(format!("timestamp is not coercable to {}", db_type_id.get_name())))
        }
    }
}
