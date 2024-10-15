use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::catalog::Schema;

pub struct IndexMetadata {
    /// The name of the index
    name: String,

    /// The name of the table on which the index is created
    table_name: String,

    /// The mapping relation between key schema and tuple schema
    key_attrs: Vec<u32>,

    /// The schema of the indexed key
    key_schema: Arc<Schema>,

    /// Is primary key?
    is_primary_key: bool,
}

impl IndexMetadata {
    /// Construct a new IndexMetadata instance.
    ///
    /// # Arguments
    /// - `index_name`: The name of the index
    /// - `table_name`: The name of the table on which the index is created
    /// - `tuple_schema`: The schema of the indexed key
    /// - `key_attrs`: The mapping from indexed columns to base table columns
    ///
    pub fn new(index_name: String, table_name: String, tuple_schema: Arc<Schema>,
               key_attrs: &[u32], is_primary_key: bool) -> Self {
        Self {
            name: index_name,
            table_name,
            key_schema: Arc::new(tuple_schema.copy_schema(key_attrs)),
            key_attrs: key_attrs.to_vec(),
            is_primary_key,
        }
    }

    /// The name of the index
    pub fn get_name(&self) -> &str { self.name.as_str() }

    /// The name of the table on which the index is created
    pub fn get_table_name(&self) -> &str { self.table_name.as_str() }


    /// A schema object pointer that represents the indexed key
    pub fn get_key_schema(&self) -> Arc<Schema> { self.key_schema.clone() }

    ///
    /// @return The number of columns inside index key (not in tuple key)
    ///
    /// NOTE: this must be defined inside the cpp source file because it
    /// uses the member of `catalog::Schema` which is not known here.
    ///
    pub fn get_index_column_count(&self) -> u32 {
        self.key_attrs.len() as u32
    }

    /// The mapping relation between indexed columns and base table columns
    pub fn get_key_attrs(&self) -> &Vec<u32> {
        &self.key_attrs
    }

    /// @return is primary key
    pub fn is_primary_key(&self) -> bool {
        self.is_primary_key
    }
}

impl Debug for IndexMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IndexMetadata[Name = {}, Type = B+Tree, Table name = {} ] :: {}", self.name, self.table_name, self.key_schema)
    }
}
