use data_types::{DBTypeId, Value};
use std::fmt::{Debug, Display, Formatter};
use catalog_schema::Schema;
use common::PageKey;
use tuple::Tuple;

/**
 * Generic key is used for indexing with opaque data.
 *
 * This key type uses an fixed length array to hold data for indexing
 * purposes, the actual size of which is specified and instantiated
 * with a template argument.
 */
// TODO - remove this derive
#[derive(PartialEq, Clone, Hash, Copy)]
pub struct GenericKey<const KEY_SIZE: usize> {
    // actual location of data, extends past the end.
    // TODO - how can it pass past the array
    pub data: [u8; KEY_SIZE],
}

impl<const KEY_SIZE: usize> GenericKey<KEY_SIZE> {
    pub fn set_from_key(&mut self, tuple: &Tuple) {
        // initialize to 0
        self.data.fill(0);
        self.data[..tuple.get_length() as usize].copy_from_slice(tuple.get_data());
    }

    /// NOTE: for test purpose only
    #[allow(unused)]
    pub fn set_from_integer(&mut self, key: i64) {
        self.data.fill(0);

        self.data[..size_of_val(&key)].copy_from_slice(key.to_ne_bytes().as_slice());
    }

    /// NOTE: for test purpose only
    #[allow(unused)]
    pub fn new_from_integer(key: i64) -> Self {
        let mut generic_key = Self::default();

        generic_key.set_from_integer(key);

        generic_key
    }

    pub(crate) fn to_value(&self, schema: &Schema, column_idx: u32) -> Value {
        let data_slice: &[u8];
        let col = schema.get_column(column_idx);
        let column_type: DBTypeId = col.get_type();
        let is_inlined = col.is_inlined();

        if is_inlined {
            data_slice = &self.data[col.get_offset() as usize..];
        } else {
            // TODO - how can offset be negative??
            let offset = i32::from_be_bytes(self.data[col.get_offset() as usize..].try_into().unwrap());
            if offset < 0 {
                unreachable!("Offset cant be below 0 {}", offset)
            }
            data_slice = &self.data[offset as usize..];
        }

        Value::deserialize_from_slice(column_type, data_slice)
    }

    /// NOTE: for test purpose only
    /// interpret the first 8 bytes as int64_t from data vector
    fn to_string(&self) -> i64 {
        i64::from_be_bytes(self.data.as_slice().try_into().unwrap())
    }
}

/// NOTE: for test purpose only
/// interpret the first 8 bytes as int64_t from data vector
impl<const KEY_SIZE: usize> Debug for GenericKey<KEY_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// NOTE: for test purpose only
/// interpret the first 8 bytes as int64_t from data vector
impl<const KEY_SIZE: usize> Display for GenericKey<KEY_SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl<const KEY_SIZE: usize> Default for GenericKey<KEY_SIZE> {
    fn default() -> Self {
        GenericKey {
            data: [0; KEY_SIZE]
        }
    }
}

impl<const KEY_SIZE: usize> PageKey for GenericKey<KEY_SIZE> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        let key: GenericKey<8> = GenericKey::default();

        assert_eq!(key.data, [0; 8])
    }
}
