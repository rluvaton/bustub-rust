use rid::RID;
use std::sync::Arc;
use table::TableHeap;
use transaction::Transaction;
use tuple::Tuple;
use crate::IndexMetadata;

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
pub trait Index where Self: 'static {
    /// Insert an entry into the index.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `rid` The RID associated with the key
    /// - `transaction` The transaction context
    ///
    /// returns `bool`: whether insertion is successful
    fn insert_entry(&self, key: &Tuple, rid: RID, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<()>;

    /// Delete an index entry by key.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `rid` The RID associated with the key (unused)
    /// - `transaction` The transaction context
    ///
    fn delete_entry(&self, key: &Tuple, rid: RID, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<()>;

    /// Search the index for the provided key.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `transaction` The transaction context
    ///
    /// returns `Vec<RID>`: The collection of RIDs with the search results
    fn scan_key(&self, key: &Tuple, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<Vec<RID>>;
    
    
    /// Verify correctness of the index
    fn verify_integrity(&self, index_metadata: &IndexMetadata, table_heap: Arc<TableHeap>);


    fn to_dyn_arc(self) -> Arc<dyn Index> where Self: Sized {
        Arc::new(self)
    }
}

