use std::sync::Arc;
use rid::RID;
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
pub trait Index {
    /// Insert an entry into the index.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `rid` The RID associated with the key
    /// - `transaction` The transaction context
    ///
    /// returns `bool`: whether insertion is successful
    fn insert_entry(&mut self, key: &Tuple, rid: RID, transaction: Arc<Transaction>) -> bool;

    /// Delete an index entry by key.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `rid` The RID associated with the key (unused)
    /// - `transaction` The transaction context
    ///
    fn delete_entry(&mut self, key: &Tuple, rid: RID, transaction: Arc<Transaction>);

    /// Search the index for the provided key.
    ///
    /// # Arguments
    /// - `key` The index key
    /// - `transaction` The transaction context
    ///
    /// returns `Vec<RID>`: The collection of RIDs with the search results
    fn scan_key(&self, key: &Tuple, transaction: Arc<Transaction>) -> Vec<RID>;
}

