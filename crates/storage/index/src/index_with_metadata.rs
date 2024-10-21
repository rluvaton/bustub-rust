use crate::{Index, IndexMetadata};
use catalog_schema::Schema;
use rid::RID;
use std::fmt::{Debug, Formatter};
use std::fs::Metadata;
use std::sync::Arc;
use table::TableHeap;
use transaction::Transaction;
use tuple::Tuple;

/// class Index - Base class for derived indices of different types
///
/// The index structure majorly maintains information on the schema of the
/// underlying table and the mapping relation between index key
/// and tuple key, and provides an abstracted way for the external world to
/// interact with the underlying index implementation without exposing
/// the actual implementation's interface.
///
/// Index object also handles predicate scan, in addition to simple insert,
/// delete, predicate insert, point query, and full index scan. Predicate scan
/// only supports conjunction, and may or may not be optimized depending on
/// the type of expressions inside the predicate.
///
pub struct IndexWithMetadata {
    /// The Index structure owns its metadata
    metadata: Arc<IndexMetadata>,
    index: Arc<dyn Index>
}

impl IndexWithMetadata {
    pub fn new(index: Arc<dyn Index>, metadata: Arc<IndexMetadata>) -> Self {
        Self {
            index,
            metadata,
        }
    }

    /// A non-owning pointer to the metadata object associated with the index
    pub fn get_metadata(&self) -> Arc<IndexMetadata> {
        self.metadata.clone()
    }

    /// @return The number of indexed columns
    pub fn get_index_column_count(&self) -> u32 {
        self.metadata.get_index_column_count()
    }

    /// @return The index name
    pub fn get_name(&self) -> &str {
        self.metadata.get_name()
    }

    /// @return The index key schema
    pub fn get_key_schema(&self) -> Arc<Schema> {
        self.metadata.get_key_schema()
    }

    /// @return The index key attributes
    pub fn get_key_attrs(&self) -> &Vec<u32> {
        self.metadata.get_key_attrs()
    }

}

impl Index for IndexWithMetadata {
    fn insert_entry(&self, key: &Tuple, rid: RID, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<()> {
        self.index.insert_entry(key, rid, transaction)
    }

    fn delete_entry(&self, key: &Tuple, rid: RID, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<()> {
        self.index.delete_entry(key, rid, transaction)
    }

    fn scan_key(&self, key: &Tuple, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<Vec<RID>> {
        self.index.scan_key(key, transaction)
    }

    fn verify_integrity(&self, _index_metadata: &IndexMetadata, table_heap: &TableHeap) {
        self.index.verify_integrity(&self.metadata, table_heap)
    }
}

impl Debug for IndexWithMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "INDEX: ({}) {:?}", self.get_name(), self.get_metadata())
    }
}
