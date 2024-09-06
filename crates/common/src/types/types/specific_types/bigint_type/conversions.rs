use crate::types::{DBTypeIdImpl, DBTypeIdTrait, StorageDBTypeTrait};
use crate::types::types::specific_types::bigint_type::base::BigIntType;
use crate::types::types::specific_types_trait::ConversionDBTypeTrait;

impl From<i64> for BigIntType {
    fn from(value: i64) -> Self {
        BigIntType::new(value)
    }
}

impl From<&[u8]> for BigIntType {
    fn from(value: &[u8]) -> Self {
        // TODO - should we have type that indicate whether it's big int or other type?
        BigIntType::deserialize_from(value)
    }
}

impl Into<DBTypeIdImpl> for BigIntType {
    fn into(self) -> DBTypeIdImpl {
        DBTypeIdImpl::BIGINT(self)
    }
}

impl ConversionDBTypeTrait for BigIntType {

    fn to_string(&self) -> String {
        // TODO - what about null
        self.value.to_string()
    }

    fn serialize_to(&self, storage: &mut [u8]) {
        storage[0..Self::SIZE as usize].copy_from_slice(self.value.to_ne_bytes().as_slice())
    }

    fn deserialize_from(storage: &[u8]) -> Self {
        BigIntType::new(i64::from_ne_bytes(storage[..Self::SIZE as usize].try_into().unwrap()))
    }
}
