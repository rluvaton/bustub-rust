use catalog_schema::Schema;
use common::PageKey;
use data_types::{Value, BUSTUB_VALUE_NULL};
use rid::RID;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::sync::Arc;

///
/// Tuple format:
/// ---------------------------------------------------------------------
/// | FIXED-SIZE or VARIED-SIZED OFFSET | PAYLOAD OF VARIED-SIZED FIELD |
/// ---------------------------------------------------------------------
///
/// TODO - implement src/include/storage/table/tuple.h
#[derive(Clone, Debug, Hash)]
pub struct Tuple {
    rid: RID,
    data: Vec<u8>,
}

impl Tuple {
    // table heap tuple
    pub fn new(rid: RID) -> Self {
        Self {
            rid,
            data: vec![],
        }
    }


    // constructor for creating a new tuple based on input value
    pub fn from_value(values: Vec<Value>, schema: Arc<Schema>) -> Self {
        assert_eq!(values.len(), schema.get_column_count());

        // 1. Calculate the size of the tuple.
        let tuple_size = schema
            .get_unlined_columns()
            .iter()
            .map(|&i| {
                let len = values[i as usize].len();

                if len == BUSTUB_VALUE_NULL {
                    0
                } else {
                    len as usize
                }
            })
            .fold(schema.get_length() as usize, |size, len| size + len + size_of::<u32>());

        // 2. Allocate memory.
        let mut data = vec![0u8; tuple_size];

        // 3. Serialize each attribute based on the input value.
        let column_count = schema.get_column_count();
        let mut offset = schema.get_length();

        for i in 0..column_count {
            let col = schema.get_column(i);

            if !col.is_inlined() {
                // Serialize relative offset, where the actual varchar data is stored.
                data[col.get_offset()..].copy_from_slice(&u32::to_ne_bytes(offset));

                // Serialize varchar value, in place (size+data).
                values[i].serialize_to(&mut data[offset as usize..]);

                let mut len = values[i].len();

                if len == BUSTUB_VALUE_NULL {
                    len = 0;
                }

                offset += len + size_of::<u32>() as u32;
            } else {
                values[i].serialize_to(&mut data[col.get_offset()..]);
            }
        }

        Self {
            data,
            rid: RID::default(),
        }
    }


    // return RID of current tuple
    pub fn get_rid(&self) -> RID {
        self.rid
    }

    // set RID of current tuple
    pub fn et_rid(&mut self, rid: RID) {
        self.rid = rid;
    }

    // Get the address of this tuple in the table's backing store
    pub fn get_data(&self) -> &[u8] { self.data.as_slice() }

    // Get length of the tuple, including varchar length
    pub fn get_length(&self) -> u32 {
        self.data.len() as u32
    }

    // Get the value of a specified column (const)
    // checks the schema to see how to return the Value.
    pub fn get_value(&self, schema: Arc<Schema>, column_idx: usize) -> Value {
        let column_type = schema.get_column(column_idx).get_type();
        let data_ptr = self.get_data_ptr(schema, column_idx);
        // the third parameter "is_inlined" is unused
        Value::deserialize_from_slice(column_type, data_ptr)
    }

    // Generates a key tuple given schemas and attributes
    pub fn key_from_tuple(&self, schema: Arc<Schema>, key_schema: Arc<Schema>, key_attrs: &[u32]) -> Self {
        Self::from_value(
            key_attrs
                .iter()
                .map(|&idx| self.get_value(schema.clone(), idx as usize))
                .collect(),
            key_schema
        )
    }

    // Is the column value null ?
    pub fn is_null(&self, schema: Arc<Schema>, column_idx: usize) -> bool {
        let value = self.get_value(schema, column_idx);
        value.is_null()
    }

    pub fn to_string(&self, schema: Arc<Schema>) -> String {
        let column_count = schema.get_column_count();

        // TODO - change to use formatter
        let cols = (0..column_count)
            .map(|idx| {
                if self.is_null(schema.clone(), idx) {
                    "<NULL>".to_string()
                } else {
                    let val = self.get_value(schema.clone(), idx);

                    format!("{}", val)
                }
            })
            .reduce(|mut output, col| {
                output.push_str(",");
                output.push_str(col.as_str());

                output
            })
            .unwrap_or("".to_string());

        format!("({cols})")
    }

    pub fn is_tuple_content_equal(a: &Self, b: &Self) -> bool { a.data == b.data }

    // Get the starting storage address of specific column
    fn get_data_ptr(&self, schema: Arc<Schema>, column_idx: usize) -> &[u8] {
        let col = schema.get_column(column_idx);

        // For inline type, data is stored where it is.
        if col.is_inlined() {
            &self.data[col.get_offset()..]
        } else {
            // We read the relative offset from the tuple data.
            let data_offset = &self.data[col.get_offset()..col.get_offset() + size_of::<i32>()];
            let offset = i32::from_ne_bytes(data_offset.try_into().unwrap());

            // And return the beginning address of the real data for the VARCHAR type.
            &self.data[(offset as usize)..]
        }
    }

    // serialize tuple data
    pub fn serialize_to(&self, dest: &mut [u8]) {
        let data_offset = size_of::<i32>();
        let size = self.data.len() as i32;
        dest[0..data_offset].copy_from_slice(size.to_ne_bytes().as_slice());
        dest[data_offset..data_offset + (size as usize)].copy_from_slice(self.data.as_slice());
    }
}


// deserialize tuple data(deep copy)
impl From<&[u8]> for Tuple {
    fn from(value: &[u8]) -> Self {
        let data_offset = size_of::<i32>();

        let size = i32::from_ne_bytes(value[..size_of::<i32>()].try_into().unwrap());

        Self {
            rid: RID::default(),
            data: value[data_offset..data_offset + size as usize].to_vec()
        }
    }
}


impl PageKey for Tuple {}


// to create a dummy tuple
impl Default for Tuple {
    fn default() -> Self {
        Self {
            rid: RID::default(),
            data: vec![],
        }
    }
}
