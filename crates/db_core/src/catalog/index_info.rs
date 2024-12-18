use std::ops::Deref;
use crate::catalog::IndexType;
use catalog_schema::Schema;
use common::config::IndexOID;
use std::sync::Arc;
use index::{Index, IndexWithMetadata};
use table::TableHeap;
use transaction::Transaction;

/// The IndexInfo class maintains metadata about a index.
#[derive(Debug)]
pub struct IndexInfo {
    /// The schema for the index key
    key_schema: Arc<Schema>,

    /// The name of the index
    name: String,

    /// An owning pointer to the index
    index: IndexWithMetadata,

    /// The unique OID for the index
    index_oid: IndexOID,

    /// The name of the table on which the index is created
    table_name: String,

    /// The size of the index key, in bytes
    #[allow(dead_code)]
    key_size: usize,

    /// Is primary key index?
    #[allow(dead_code)]
    is_primary_key: bool,

    /// The index type
    #[allow(dead_code)]
    index_type: IndexType,
}

impl IndexInfo {
    pub fn new(
        key_schema: Arc<Schema>,
        name: String,
        index: IndexWithMetadata,
        index_oid: IndexOID,
        table_name: String,
        key_size: usize,
        is_primary_key: bool,
        index_type: IndexType,
    ) -> Self {
        Self {
            key_schema,
            name,
            index,
            index_oid,
            table_name,
            key_size,
            is_primary_key,
            index_type,
        }
    }

    // -----------------------------
    // For bustub instance debugging
    // -----------------------------


    pub fn get_index_oid(&self) -> IndexOID {
        self.index_oid
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_table_name(&self) -> &String {
        &self.table_name
    }

    pub fn get_key_schema(&self) -> Arc<Schema> {
        self.key_schema.clone()
    }
    
    pub fn verify_integrity(&self, table_heap: &TableHeap, txn: &Transaction) {
        self.index.verify_integrity(self.index.get_metadata().deref(), table_heap, txn)
    }
    
    pub fn get_index(&self) -> &IndexWithMetadata {
        &self.index
    }

    pub fn delete_completely(self, transaction: &Transaction) -> error_utils::anyhow::Result<()> {
        self.index.delete_completely(transaction)
    }
}
