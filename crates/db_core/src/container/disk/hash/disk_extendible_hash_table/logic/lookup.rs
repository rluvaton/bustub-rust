use super::super::type_alias_trait::TypeAliases;
use super::super::HashTable;
use crate::concurrency::Transaction;
use crate::container::hash::KeyHasher;
use crate::storage::Comparator;
use pages::{PageId, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use error_utils::Context;
use std::fmt::Debug;
use std::sync::Arc;
use buffer_common::AccessType;
use buffer_pool_manager::BufferPool;
use buffer_pool_manager::errors::{BufferPoolError, MapErrorToBufferPoolError};

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum LookupError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] BufferPoolError),
}


impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    const NOTHING_FOUND: Vec<Value> = vec![];

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
    pub fn get_value(&self, key: &Key, transaction: Option<Arc<Transaction>>) -> Result<Vec<Value>, LookupError> {
        // TODO - use transaction
        assert!(transaction.is_none(), "transaction is not none, transactions are not supported at the moment");

        let directory_page_id: PageId;
        let bucket_page_id: PageId;

        // 1. Hash key as most probably the hash table is initialized,
        //    and we want to avoid holding the header page read guard while hashing (even though it's fast)
        let key_hash = self.hash(key);

        {
            // 2. Get the header page
            let header = self.bpm.fetch_page_read(self.header_page_id, AccessType::Unknown)
                .map_err_to_buffer_pool_err()
                .context("Failed to fetch header")?;

            let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

            // 3. Find the directory page id where the value might be
            let directory_index = header_page.hash_to_directory_index(key_hash);
            directory_page_id = header_page.get_directory_page_id(directory_index);
        } // Drop header page guard

        // 4. If we got invalid page than the directory is missing
        if directory_page_id == INVALID_PAGE_ID {
            return Ok(Self::NOTHING_FOUND);
        }

        {
            // 5. Get the directory page
            let directory = self.bpm.fetch_page_read(directory_page_id, AccessType::Unknown).map_err_to_buffer_pool_err()?;

            let directory_page = directory.cast::<<Self as TypeAliases>::DirectoryPage>();

            // 6. Find the bucket page id where the value might be
            let bucket_index = directory_page.hash_to_bucket_index(key_hash);
            bucket_page_id = directory_page.get_bucket_page_id(bucket_index)
        } // Release directory page guard

        // 7. If we got invalid page than the bucket is missing
        if bucket_page_id == INVALID_PAGE_ID {
            return Ok(Self::NOTHING_FOUND);
        }

        let found_value: Option<Value>;

        {
            // 8. Get the bucket page
            let bucket = self.bpm.fetch_page_read(bucket_page_id, AccessType::Unknown).map_err_to_buffer_pool_err()?;

            let bucket_page = bucket.cast::<<Self as TypeAliases>::BucketPage>();

            // 9. Lookup the value for the key in the target bucket
            found_value = bucket_page.lookup(key, &self.cmp)
                // Clone the value before releasing the page guard as we hold reference to something that will be freed
                .copied();
        } // Drop bucket page guard


        Ok(found_value.map_or_else(

            // In case None, return empty results
            || Self::NOTHING_FOUND,

            // In case found return that result
            |v| vec![v],
        ))
    }
}


