use catalog_schema::Schema;
use common::PageKey;
use data_types::Value;
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
        todo!()
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
    pub fn get_value(&self, schema: Arc<Schema>, column_idx: u32) -> Value {
        todo!()
    }

    // Generates a key tuple given schemas and attributes
    pub fn key_from_tuple(schema: Arc<Schema>, key_schema: Arc<Schema>, key_attrs: &[u32]) -> Self {
        todo!()
    }

    // Is the column value null ?
    pub fn is_null(&self, schema: Arc<Schema>, column_idx: u32) -> bool {
        let value = self.get_value(schema, column_idx);
        value.is_null()
    }

    pub fn to_string(&self, schema: Arc<Schema>) -> String {
        todo!()
    }

    pub fn is_tuple_content_equal(a: &Self, b: &Self) -> bool { a.data == b.data }
}


// serialize tuple data
impl From<&[u8]> for Tuple {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}

// deserialize tuple data(deep copy)
impl From<Tuple> for &[u8] {
    fn from(value: Tuple) -> Self {
        todo!()
    }
}


impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
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
