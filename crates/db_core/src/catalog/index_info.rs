use crate::catalog::{IndexType, Schema};
use crate::storage::Index;
use common::config::IndexOID;
use std::sync::Arc;

/// The IndexInfo class maintains metadata about a index.
pub struct IndexInfo {

    /// The schema for the index key
    #[allow(unused)]
    key_schema: Schema,

    /// The name of the index
    #[allow(unused)]
    name: String,

    /// An owning pointer to the index
    #[allow(unused)]
    index: Arc<Index>,

    /// The unique OID for the index
    #[allow(unused)]
    index_oid: IndexOID,

    /// The name of the table on which the index is created
    #[allow(unused)]
    table_name: String,

    /// The size of the index key, in bytes
    #[allow(unused)]
    key_size: usize,

    /// Is primary key index?
    #[allow(unused)]
    is_primary_key: bool,

    /// The index type
    /// Default: `IndexType::BPlusTreeIndex`
    #[allow(unused)]
    index_type: IndexType,
}
