use super::super::type_alias_trait::TypeAliases;
use super::super::HashTable;
use crate::buffer;
use crate::buffer::PinWritePageGuard;
use crate::concurrency::Transaction;
use crate::container::hash::KeyHasher;
use crate::storage::Comparator;
use common::config::{PageId, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use error_utils::Context;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use crate::buffer::errors::{BufferPoolError, MapErrorToBufferPoolError};

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum RemoveError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::errors::BufferPoolError),

    #[error("error during merge")]
    MergeError(#[from] MergeError),
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum MergeError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::errors::BufferPoolError),

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
        let mut header = self.bpm.fetch_page_write(self.header_page_id).map_err_to_buffer_pool_err().context("Hash Table header page must exists when trying to insert")?;

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
        let mut directory = self.bpm.fetch_page_write(directory_page_id).map_err_to_buffer_pool_err().context("Directory page should exists")?;

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 6. Find the bucket page id where the value might be
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        // 7. If no bucket exists it means that the value is missing
        if bucket_page_id == INVALID_PAGE_ID {
            return Ok(false);
        }

        // 8. Get the bucket page
        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).map_err_to_buffer_pool_err().context("Failed to fetch bucket page")?;
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

    fn trigger_merge<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32, key_hash: u32) -> Result<(), MergeError> {
        // Try to merge the buckets
        self.try_merge(directory_page_guard, bucket_page_guard, bucket_index, key_hash)
    }

    fn try_merge<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, mut bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32, key_hash: u32) -> Result<(), MergeError> {
        todo!()
        // let mut directory_page = directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();
        // let mut bucket_page = bucket_page_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();
        //
        // // 2. Make sure need to merge
        // assert!(bucket_page.is_empty(), "page must be empty before merging");
        //
        // // 3. Create new bucket to be the new split bucket
        // let new_bucket = self.init_new_bucket().context("Failed to initialize new bucket page when trying to split bucket")?;
        // let new_bucket_page_id = new_bucket.get_page_id();
        // let mut new_bucket_guard = new_bucket.upgrade_write();
        //
        // // self.
        //
        // // 4. Shrink the directory if needed
        // if directory_page.can_shrink() {
        //     let shrunk = directory_page.decr_global_depth();
        //
        //     // Directory is now empty
        //     if !shrunk {
        //         return Ok(())
        //     }
        // }
        //
        // // 5. Split bucket
        // self.split_local_bucket(bucket_index, &mut directory_page, &mut bucket_page, &mut new_bucket_guard);
        //
        // // 6. Find out which bucket to insert to after the split
        // let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();
        //
        // let bucket_index_to_insert = directory_page.hash_to_bucket_index(key_hash);
        // let bucket_to_insert_page_id = directory_page.get_bucket_page_id(bucket_index_to_insert);
        // let bucket_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_page } else { bucket_page };
        //
        // // 7. Check if still after the split we can't insert
        // if bucket_to_insert.is_full() {
        //     let bucket_guard_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_guard } else { bucket_page_guard };
        //
        //     // 7.1 Split again with the current bucket that is full (The bucket index is always the one that about to overflow)
        //     return self.try_split(directory_page_guard, bucket_guard_to_insert, bucket_index_to_insert, key_hash, tries_left - 1);
        // }
        //
        // let bucket_guard_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_guard } else { bucket_page_guard };
        //
        // Ok(bucket_guard_to_insert)
    }

    /// Return the splitted bucket indices
    fn shrink_local_bucket(&mut self, mut bucket_index: u32, directory_page: &mut <Self as TypeAliases>::DirectoryPage, bucket_page_to_split: &mut <Self as TypeAliases>::BucketPage, new_bucket_guard: &mut PinWritePageGuard) {
        todo!();
        //
        // let new_bucket_page_id = new_bucket_guard.get_page_id();
        // let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();
        //
        // // 1. Change bucket index to be the first bucket of the specific page, so it will be the index of the bucket that will be kept as is
        // let new_bucket_index = bucket_index.turn_on_bit(directory_page.get_local_depth(bucket_index) as usize + 1);
        //
        // // 2. Trim bucket index to the first index that point to the bucket
        // bucket_index = bucket_index & directory_page.get_local_depth_mask(bucket_index);
        //
        // assert_ne!(bucket_index, new_bucket_index, "Bucket index cannot be the same as the new bucket index");
        //
        // // 3. Register the new bucket in the directory
        // directory_page.set_bucket_page_id(new_bucket_index, new_bucket_page_id);
        //
        // // 4. Update local length for both buckets
        // directory_page.incr_local_depth(bucket_index);
        // directory_page.incr_local_depth(new_bucket_index);
        //
        // // 5. Rehash all current bucket page content and find the correct bucket
        // let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_split
        //     .iter()
        //     .partition(|(key, _)| directory_page.hash_to_bucket_index(self.hash(key)) == new_bucket_index);
        //
        // // 6. set the current bucket items in the new location
        // // Optimization: Only if not empty as if nothing to add to the new bucket than it means as is
        // if !new_bucket_items.is_empty() {
        //     new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
        //     bucket_page_to_split.replace_all_entries(current_bucket_items.as_slice());
        // }
    }

    fn remove_directory(&mut self, page_id: PageId) -> Result<(), BufferPoolError> {
        // TODO - remove page and unlink
        let deleted = self.bpm.delete_page(page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty bucket page")?;

        Ok(())

    }

    fn remove_bucket(&mut self, page_id: PageId) -> Result<(), BufferPoolError> {
        // TODO - remove page and unlink
        let deleted = self.bpm.delete_page(page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty bucket page")?;

        Ok(())
    }


}

