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
use binary_utils::ModifyBit;
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

        let mut directory_index: u32;
        let mut directory_page_id: PageId;
        let mut bucket_page_id: PageId;

        // 1. Hash key
        let key_hash = self.hash(key);

        // 2. Get the header page
        // TODO - get the page as read and upgrade if needed as most of the time the header page exists as well as the directory page
        let mut header = self.bpm.fetch_page_write(self.header_page_id).map_err_to_buffer_pool_err().context("Hash Table header page must exists when trying to insert")?;

        {
            let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();

            // 3. Find the directory page id where the value might be
            directory_index = header_page.hash_to_directory_index(key_hash);
            directory_page_id = header_page.get_directory_page_id(directory_index);
        }

        // 4. If no directory exists it means that the value is missing
        if directory_page_id == INVALID_PAGE_ID {
            return Ok(false);
        }

        // 5. Get the directory page
        // TODO - get the page as read and upgrade if needed?
        let mut directory = self.bpm.fetch_page_write(directory_page_id).map_err_to_buffer_pool_err().context("Directory page should exists")?;

        {
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
                self.trigger_merge(&mut directory, bucket, bucket_index)?;
            }
        }

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 12. If after the merge the directory is empty
        if directory_page.size() == 0 {
            // 13. Remove directory from header
            // TODO - remove the page as well
            // TODO - Do not remove the directory page and keep it, and when doing some compaction or GC claim that page
            self.remove_directory(&mut header, directory, directory_index).context("Failed to remove directory")?;

            // 14. Unregister directory page
            let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();
            header_page.set_directory_page_id(directory_index, INVALID_PAGE_ID);
        }

        Ok(true)
    }

    fn trigger_merge<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32) -> Result<(), MergeError> {
        // Try to merge the buckets
        self.try_merge(directory_page_guard, bucket_page_guard, bucket_index)
    }

    fn try_merge<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, mut bucket_page_guard: PinWritePageGuard<'a>, mut bucket_index: u32) -> Result<(), MergeError> {
        let mut directory_page = directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();
        let mut bucket_page = bucket_page_guard.cast::<<Self as TypeAliases>::BucketPage>();

        // 1. Make sure need to merge
        assert!(bucket_page.is_empty(), "page must be empty before merging");

        // 2. Shrink the directory if needed
        if directory_page.can_shrink() {
            let shrunk = directory_page.decr_global_depth();

            // Directory is now empty
            if !shrunk {
                return Ok(())
            }
        }

        let bucket_local_depth = directory_page.get_local_depth(bucket_index);

        // 3. If the bucket depth is now 0 it means that we removed everything
        if bucket_local_depth == 0 {
            // 4. Remove bucket
            self.remove_bucket(directory_page_guard, bucket_page_guard, bucket_index)?;

            return Ok(());
        }


        // 5. Trim bucket index to the first index that point to the bucket
        bucket_index = bucket_index & directory_page.get_local_depth_mask(bucket_index);

        // 6. Get bucket index of the previous bucket so it will be the bucket we will merge into
        let bucket_index_to_merge_into = bucket_index.turn_off_bit(bucket_local_depth as usize);

        assert_ne!(bucket_index, bucket_index_to_merge_into, "Bucket index cant be the one to merge into");

        directory_page.decr_local_depth(bucket_index_to_merge_into);

        let bucket_page_id_to_merge_into = directory_page.get_bucket_page_id(bucket_index_to_merge_into);

        // 7. Get the bucket page
        let mut bucket_guard_to_merge_into = self.bpm.fetch_page_write(bucket_page_id_to_merge_into).map_err_to_buffer_pool_err().context("Failed to fetch bucket page")?;
        let bucket_to_merge_into = bucket_guard_to_merge_into.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 8. Move all bucket items to the bucket to merge into
        // Safety: this is safe as we are inside merge buckets
        unsafe { bucket_to_merge_into.merge_bucket(bucket_page_guard) }

        // 9. Check if still after the merge the bucket is still empty
        if bucket_to_merge_into.is_empty() {
            // 10. Try to merge current bucket as well
            return self.try_merge(directory_page_guard, bucket_guard_to_merge_into, bucket_index_to_merge_into);
        }

        Ok(())
    }

    /// Return the splitted bucket indices
    fn merge_local_bucket(&mut self, mut bucket_index: u32, directory_page: &mut <Self as TypeAliases>::DirectoryPage, bucket_page_to_merge: &mut <Self as TypeAliases>::BucketPage, bucket_page_guard_to_merge_into: &mut PinWritePageGuard) {
        let bucket_local_depth = directory_page.get_local_depth(bucket_index);

        // 1. If bucket local depth is 0 we can't merge anymore
        if bucket_local_depth == 0 {
            return
        }

        let new_bucket_page_id = bucket_page_guard_to_merge_into.get_page_id();
        let new_bucket_page = bucket_page_guard_to_merge_into.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 2. Change bucket index to be the last bucket of the specific page, so it will be the index of the bucket that will be kept as is
        let bucket_index_to_merge_into = bucket_index.turn_off_bit(bucket_local_depth as usize);

        // 2. Trim bucket index to the first index that point to the bucket
        bucket_index = bucket_index & directory_page.get_local_depth_mask(bucket_index);

        assert_ne!(bucket_index, bucket_index_to_merge_into, "Bucket index cannot be the same as the bucket index to merge into");

        // 3. Register the new bucket in the directory
        directory_page.set_bucket_page_id(bucket_index_to_merge_into, new_bucket_page_id);

        // 4. Update local length for both buckets
        directory_page.decr_local_depth(bucket_index);
        directory_page.decr_local_depth(bucket_index_to_merge_into);

        // 5. Rehash all current bucket page content and find the correct bucket
        let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_merge
            .iter()
            .partition(|(key, _)| directory_page.hash_to_bucket_index(self.hash(key)) == bucket_index_to_merge_into);

        // 6. set the current bucket items in the new location
        // Optimization: Only if not empty as if nothing to add to the new bucket than it means as is
        if !new_bucket_items.is_empty() {
            new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
        }
    }

    fn remove_directory(&mut self, header_page_guard: &mut PinWritePageGuard, directory_page_guard: PinWritePageGuard, directory_index: u32) -> Result<(), BufferPoolError> {
        header_page_guard.cast_mut::<<Self as TypeAliases>::HeaderPage>().set_directory_page_id(directory_index, INVALID_PAGE_ID);

        let directory_page_id = directory_page_guard.get_page_id();

        // Drop the directory to be able to delete it
        drop(directory_page_guard);

        let deleted = self.bpm.delete_page(directory_page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty directory page")?;

        assert_eq!(deleted, true, "Should be able to delete page");

        Ok(())

    }

    fn remove_bucket(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: PinWritePageGuard, bucket_index: u32) -> Result<(), BufferPoolError> {
        directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>().set_bucket_page_id(bucket_index, INVALID_PAGE_ID);

        let bucket_page_id = bucket_page_guard.get_page_id();

        // Drop the bucket to be able to delete it
        drop(bucket_page_guard);

        let deleted = self.bpm.delete_page(bucket_page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty bucket page")?;

        assert_eq!(deleted, true, "Should be able to delete page");

        Ok(())
    }


}

