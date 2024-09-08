use std::sync::Arc;
use common::config::{IndexOID, TableOID};
use crate::catalog::Schema;
use crate::storage::TableHeap;

/// The IndexInfo class maintains metadata about a index.
pub struct IndexInfo {

    /// The schema for the index key
    key_schema: Schema,

    /// The name of the index
    name: String,

    /// An owning pointer to the index
    index: Arc<Index>,

    /// The unique OID for the index
    index_oid: IndexOID,

    /// The name of the table on which the index is created
    table_name: String,

    /// The size of the index key, in bytes
    key_size: usize,
    
    /// Is primary key index?
    is_primary_key: bool,

    /// The index type
    /// Default: `IndexType::BPlusTreeIndex`
    index_type: IndexType,
}
