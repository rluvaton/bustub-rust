use common::{PageKey, PageValue};
use std::sync::Arc;
use transaction::Transaction;
use crate::container::disk::hash::errors::HashTableResult;

pub trait HashTableWithSingleKey<Key: PageKey, Value: PageValue> {

    /// Inserts a key-value pair into the hash table.
    ///
    /// # Arguments
    ///
    /// - `key`: the key to create
    /// - `value`: the value to be associated with the key
    /// - `transaction`: the current transaction
    ///
    /// Returns: `anyhow::Result` with empty value if succeed or error if failed
    ///
    /// TODO - return custom result if inserted or not - NotInsertedError
    ///
    fn insert(&mut self, key: &Key, value: &Value, transaction: Option<Arc<Transaction>>) -> HashTableResult<()>;

    /// TODO(P2): Add implementation
    /// Removes a key-value pair from the hash table.
    ///
    /// # Arguments
    ///
    /// - `key`: the key to delete
    /// - `transaction`: the current transaction
    ///
    /// Returns: `true` if remove succeeded, `false` otherwise
    ///
    fn remove(&mut self, key: &Key, transaction: Option<Arc<Transaction>>) -> HashTableResult<bool>;

    /// TODO(P2): Add implementation
    /// Get the value associated with a given key in the hash table.
    ///
    /// Note(fall2023): This semester you will only need to support unique key-value pairs.
    ///
    /// # Arguments
    ///
    /// - `key`: the key to look up
    /// - `transaction`: the current transaction
    ///
    /// Returns: `Vec<Value` the value(s) associated with the given key
    ///
    fn get_value(&self, key: &Key, transaction: Option<Arc<Transaction>>) -> HashTableResult<Option<Value>>;

    /// Helper function to verify the integrity of the extendible hash table's directory.
    fn verify_integrity(&self);

    /// Helper function to print out the HashTable.
    fn print_hash_table(&self);
}
