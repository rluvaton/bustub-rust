use super::super::type_alias_trait::TypeAliases;
use super::super::DiskHashTable;
use pages::{PageId, INVALID_PAGE_ID};
use common::{Comparator, PageKey, PageValue};
use error_utils::Context;
use std::fmt::Debug;
use buffer_common::AccessType;
use buffer_pool_manager::BufferPool;
use buffer_pool_manager::errors::{BufferPoolError, MapErrorToBufferPoolError};
use hashing_common::KeyHasher;
use transaction::Transaction;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum UpdateError {
    #[error("Key is missing")]
    KeyIsMissing,

    #[error("buffer pool error")]
    BufferPoolError(#[from] BufferPoolError),
}


impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    /// TODO(P2): Add implementation
    /// Update the value of a specific key
    ///
    /// # Arguments
    ///
    /// - `key`: the key to change the value for
    /// - `value`: the new value
    /// - `transaction`: the current transaction
    ///
    /// Returns: `()` the value(s) associated with the given key
    ///
    pub fn update(&self, key: &Key, value: &Value, _transaction: &Transaction) -> Result<(), UpdateError> {
        let directory_page_id: PageId;
        let bucket_page_id: PageId;
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
            return Err(UpdateError::KeyIsMissing);
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
            return Err(UpdateError::KeyIsMissing);
        }


        // 8. Get the bucket page
        let mut bucket = self.bpm.fetch_page_write(bucket_page_id, AccessType::Unknown).map_err_to_buffer_pool_err()?;

        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 9. Lookup the value for the key in the target bucket
        let replaced = bucket_page.replace(key, &value, &self.cmp);

        if !replaced {
            return Err(UpdateError::KeyIsMissing);
        }

        Ok(())
    }
}


