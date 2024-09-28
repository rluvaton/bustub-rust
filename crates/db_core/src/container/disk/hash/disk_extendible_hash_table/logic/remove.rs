use super::super::type_alias_trait::TypeAliases;
use super::super::HashTable;
use crate::buffer;
use crate::concurrency::Transaction;
use crate::container::hash::KeyHasher;
use crate::storage::Comparator;
use common::config::{PageId, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use error_utils::Context;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum RemoveError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown error")]
    Unknown,
}

const NUMBER_OF_MERGE_RETRIES: usize = 3;

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{

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
    pub fn remove(&mut self, key: &Key, transaction: Option<Arc<Transaction>>) -> Result<bool, RemoveError> {
        // TODO - use transaction
        assert!(transaction.is_none(), "transaction is not none, transactions are not supported at the moment");

        // TODO - performance improvement release write latch as soon as can

        let mut directory_page_id: PageId;
        let mut bucket_page_id: PageId;

        // 1. Hash key
        let key_hash = self.hash(key);

        // 2. Get the header page
        // TODO - get the page as read and upgrade if needed as most of the time the header page exists as well as the directory page
        let mut header = self.bpm.fetch_page_write(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;

        let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        // 3. Find the directory page id where the value might be
        let directory_index = header_page.hash_to_directory_index(key_hash);
        directory_page_id = header_page.get_directory_page_id(directory_index);

        // 4. If no directory exists it means that the value is missing
        if directory_page_id == INVALID_PAGE_ID {
            return Ok(false);
        }

        // 5. Get the directory page
        // TODO - get the page as read and upgrade if needed?
        let mut directory = self.bpm.fetch_page_write(directory_page_id).context("Directory page should exists")?;

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 6. Find the bucket page id where the value might be
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        // 7. If no bucket exists it means that the value is missing
        if bucket_page_id == INVALID_PAGE_ID {
            return Ok(false);
        }

        // 8. Get the bucket page
        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).context("Failed to fetch bucket page")?;
        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 9. Try to delete the value
        let removed = bucket_page.remove(&key, &self.cmp);

        // 10. If not removed this means that the value was not found
        if !removed {
            return Ok(false)
        }

        // 11. If bucket is empty, need to merge
        if bucket_page.is_empty() {
            // TODO - pass the header as well as we might need to remove the directory as well
            self.trigger_merge(&mut directory, bucket, bucket_index, key_hash)?;
        }

        // 12. If after the merge the directory is empty
        if directory_page.size() == 0 {
            // 13. Remove directory from header
            // TODO - remove the page as well
            self.remove_directory()
        }

        Ok(true)
    }


}

