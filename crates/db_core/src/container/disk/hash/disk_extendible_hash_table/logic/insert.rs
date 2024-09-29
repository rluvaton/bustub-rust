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
use binary_utils::ModifyBit;
use crate::buffer::{PinPageGuard, PinWritePageGuard};
use crate::buffer::errors::MapErrorToBufferPoolError;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InsertionError {
    #[error("Key already exists")]
    KeyAlreadyExists,

    #[error("Tried to split bucket for {0} times")]
    ReachedSplitRetryLimit(usize),

    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::errors::BufferPoolError),

    #[error("No space left for inserting as the bucket is full and it cannot be splitted again")]
    BucketIsFull,
}

const NUMBER_OF_SPLIT_RETRIES: usize = 3;

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{

    /// TODO(P2): Add implementation
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
    pub fn insert(&mut self, key: &Key, value: &Value, transaction: Option<Arc<Transaction>>) -> Result<(), InsertionError> {
        // TODO - use transaction
        assert!(transaction.is_none(), "transaction is not none, transactions are not supported at the moment");

        // TODO - performance improvement release write latch as soon as can

        let mut directory_page_id: PageId;
        let mut bucket_page_id: PageId;

        // 1. Hash key
        let key_hash = self.hash(key);

        // 2. Get the header page
        // TODO - get the page as read and upgrade if needed as most of the time the header page exists as well as the directory page
        let mut header = self.bpm.fetch_page_write(self.header_page_id).map_err_to_buffer_pool_err().context("Hash Table header page must exists when trying to insert")?;

        let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        // 3. Find the directory page id where the value might be
        let directory_index = header_page.hash_to_directory_index(key_hash);
        directory_page_id = header_page.get_directory_page_id(directory_index);

        // 4. If no directory exists create it
        if directory_page_id == INVALID_PAGE_ID {
            let directory_guard = self.init_new_directory()
                .context("Failed to initialize new directory page while trying to insert new entry to the hash table")?;
            directory_page_id = directory_guard.get_page_id();

            // 5. Register the directory in the header page
            header_page.set_directory_page_id(directory_index, directory_page_id);
        } // Drop the new directory - on purpose we don't keep it for simplicity

        // 6. Get the directory page
        // TODO - get the page as read and upgrade if needed?
        let mut directory = self.bpm.fetch_page_write(directory_page_id).map_err_to_buffer_pool_err().context("Directory page should exists")?;

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 7. Find the bucket page id where the value might be
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        // 8. If no bucket exists create it
        if bucket_page_id == INVALID_PAGE_ID {
            let bucket_guard = self.init_new_bucket().context("Failed to initialize new bucket page while trying to insert new entry to the hash table")?;
            bucket_page_id = bucket_guard.get_page_id();

            // 9. Register the bucket in the directory page
            directory_page.set_bucket_page_id(bucket_index, bucket_page_id);
        } // Drop the new bucket - on purpose we don't keep it for simplicity

        // 10. Get the bucket page
        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).map_err_to_buffer_pool_err().context("Failed to fetch bucket page")?;
        let bucket_page = bucket.cast::<<Self as TypeAliases>::BucketPage>();

        // 11. If the bucket already contain the data return error
        if bucket_page.lookup(&key, &self.cmp).is_some() {
            return Err(InsertionError::KeyAlreadyExists);
        }

        // 12. if bucket page is full, need to split
        if bucket_page.is_full() {
            bucket = self.trigger_split(&mut directory, bucket, bucket_index, key_hash)?;
        }

        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 13. try to insert the key
        // Safety: doing unwrap as it should not happen since we split
        //         and we have a lock - TODO - add a lock
        bucket_page.insert(key, value, &self.cmp).unwrap();

        Ok(())
    }

    fn trigger_split<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32, key_hash: u32) -> Result<PinWritePageGuard<'a>, InsertionError> {
        // Try to split the bucket with 3 iteration (after that it seems like the hash function is not good, or we have a bug)
        self.try_split(directory_page_guard, bucket_page_guard, bucket_index, key_hash, NUMBER_OF_SPLIT_RETRIES)
    }

    fn try_split<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, mut bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32, key_hash: u32, tries_left: usize) -> Result<PinWritePageGuard<'a>, InsertionError> {
        // 1. Check if reached max tries
        if tries_left == 0 {
            eprintln!("Trying to insert key but after split the page is still full, the hash might not evenly distribute the keys");
            return Err(InsertionError::ReachedSplitRetryLimit(NUMBER_OF_SPLIT_RETRIES));
        }

        let mut directory_page = directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();
        let mut bucket_page = bucket_page_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 2. Make sure need to split
        assert!(bucket_page.is_full(), "page must be full before splitting");

        // 3. Create new bucket to be the new split bucket
        let new_bucket = self.init_new_bucket().context("Failed to initialize new bucket page when trying to split bucket")?;
        let new_bucket_page_id = new_bucket.get_page_id();
        let mut new_bucket_guard = new_bucket.upgrade_write();

        // 4. Expand the directory if needed
        if directory_page.get_local_depth(bucket_index) == directory_page.get_global_depth() {
            let increase_result = directory_page.incr_global_depth();

            if !increase_result {
                return Err(InsertionError::BucketIsFull)
            }
        }

        // 5. Split bucket
        self.split_local_bucket(bucket_index, &mut directory_page, &mut bucket_page, &mut new_bucket_guard);

        // 6. Find out which bucket to insert to after the split
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        let bucket_index_to_insert = directory_page.hash_to_bucket_index(key_hash);
        let bucket_to_insert_page_id = directory_page.get_bucket_page_id(bucket_index_to_insert);
        let bucket_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_page } else { bucket_page };

        // 7. Check if still after the split we can't insert
        if bucket_to_insert.is_full() {
            let bucket_guard_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_guard } else { bucket_page_guard };

            // 7.1 Split again with the current bucket that is full (The bucket index is always the one that about to overflow)
            return self.try_split(directory_page_guard, bucket_guard_to_insert, bucket_index_to_insert, key_hash, tries_left - 1);
        }

        let bucket_guard_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_guard } else { bucket_page_guard };

        Ok(bucket_guard_to_insert)
    }

    /// Return the splitted bucket indices
    fn split_local_bucket(&mut self, mut bucket_index: u32, directory_page: &mut <Self as TypeAliases>::DirectoryPage, bucket_page_to_split: &mut <Self as TypeAliases>::BucketPage, new_bucket_guard: &mut PinWritePageGuard) {

        let new_bucket_page_id = new_bucket_guard.get_page_id();
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 1. Change bucket index to be the first bucket of the specific page, so it will be the index of the bucket that will be kept as is
        let new_bucket_index = bucket_index.turn_on_bit(directory_page.get_local_depth(bucket_index) as usize + 1);

        // 2. Trim bucket index to the first index that point to the bucket
        bucket_index = bucket_index & directory_page.get_local_depth_mask(bucket_index);

        assert_ne!(bucket_index, new_bucket_index, "Bucket index cannot be the same as the new bucket index");

        // 3. Register the new bucket in the directory
        directory_page.set_bucket_page_id(new_bucket_index, new_bucket_page_id);

        // 4. Update local length for both buckets
        directory_page.incr_local_depth(bucket_index);
        directory_page.incr_local_depth(new_bucket_index);

        // 5. Rehash all current bucket page content and find the correct bucket
        let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_split
            .iter()
            .partition(|(key, _)| directory_page.hash_to_bucket_index(self.hash(key)) == new_bucket_index);

        // 6. set the current bucket items in the new location
        // Optimization: Only if not empty as if nothing to add to the new bucket than it means as is
        if !new_bucket_items.is_empty() {
            new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
            bucket_page_to_split.replace_all_entries(current_bucket_items.as_slice());
        }
    }

    fn init_new_directory(&mut self) -> Result<PinPageGuard, buffer::errors::BufferPoolError> {
        let directory_page = self.bpm.new_page_guarded().map_err_to_buffer_pool_err().context("Should be able to create page")?;

        {
            let mut directory_guard = directory_page.write();
            let directory = directory_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

            directory.init(Some(self.directory_max_depth));
        }

        Ok(directory_page)
    }

    fn init_new_bucket(&mut self) -> Result<PinPageGuard, buffer::errors::BufferPoolError> {
        let bucket_page = self.bpm.new_page_guarded().map_err_to_buffer_pool_err().context("Should be able to create page")?;

        {
            let mut bucket_guard = bucket_page.write();
            let bucket = bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

            bucket.init(Some(self.bucket_max_size));
        }

        Ok(bucket_page)
    }

}
