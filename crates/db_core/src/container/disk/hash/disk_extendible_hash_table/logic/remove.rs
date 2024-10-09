use super::super::type_alias_trait::TypeAliases;
use super::super::HashTable;
use buffer_pool_manager::{BufferPool, PageWriteGuard};
use buffer_pool_manager::errors::{BufferPoolError, MapErrorToBufferPoolError};
use crate::concurrency::Transaction;
use crate::container::hash::KeyHasher;
use crate::storage::Comparator;
use binary_utils::{GetAllNumbersWithPrefixBitsUntilMaxBits, GetNBits, ModifyBit};
use pages::{PageId, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use error_utils::Context;
use std::fmt::Debug;
use std::sync::Arc;
use buffer_common::AccessType;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum RemoveError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] BufferPoolError),

    #[error("error during merge")]
    MergeError(#[from] MergeError),
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum MergeError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] BufferPoolError),

    #[error("unknown error")]
    Unknown,
}

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl>
HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
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
    pub fn remove(
        &self,
        key: &Key,
        transaction: Option<Arc<Transaction>>,
    ) -> Result<bool, RemoveError> {
        // TODO - use transaction
        assert!(
            transaction.is_none(),
            "transaction is not none, transactions are not supported at the moment"
        );

        // TODO - performance improvement release write latch as soon as can

        let directory_index: u32;
        let directory_page_id: PageId;
        let bucket_page_id: PageId;

        // 1. Hash key
        let key_hash = self.hash(key);

        // 2. Get the header page
        // TODO - get the page as read and upgrade if needed as most of the time the header page exists as well as the directory page
        let mut header = self
            .bpm
            .fetch_page_write(self.header_page_id, AccessType::Unknown)
            .map_err_to_buffer_pool_err()
            .context("Hash Table header page must exists when trying to insert")?;

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
        let mut directory = self
            .bpm
            .fetch_page_write(directory_page_id, AccessType::Unknown)
            .map_err_to_buffer_pool_err()
            .context("Directory page should exists")?;

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
            let mut bucket = self
                .bpm
                .fetch_page_write(bucket_page_id, AccessType::Unknown)
                .map_err_to_buffer_pool_err()
                .context("Failed to fetch bucket page")?;
            let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

            // 9. Try to delete the value
            let removed = bucket_page.remove(&key, &self.cmp);

            // 10. If not removed this means that the value was not found
            if !removed {
                return Ok(false);
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
            self.remove_directory(&mut header, directory, directory_index)
                .context("Failed to remove directory")?;

            // 14. Unregister directory page
            let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();
            header_page.set_directory_page_id(directory_index, INVALID_PAGE_ID);
        }

        Ok(true)
    }

    fn trigger_merge<'a>(
        &self,
        directory_page_guard: &mut PageWriteGuard,
        empty_bucket_page_guard: PageWriteGuard<'a>,
        empty_bucket_index: u32,
    ) -> Result<(), MergeError> {
        let directory_page =
            directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();
        let bucket_page = empty_bucket_page_guard.cast::<<Self as TypeAliases>::BucketPage>();

        // 1. Make sure we need to merge
        assert!(bucket_page.is_empty(), "page must be empty before merging");

        let bucket_local_depth = directory_page.get_local_depth(empty_bucket_index);

        // 2. If the bucket depth is now 0 it means that we removed everything
        if bucket_local_depth == 0 {
            // 3. Remove bucket
            self.remove_bucket(directory_page_guard, empty_bucket_page_guard, empty_bucket_index)?;

            return Ok(());
        }

        // 4. Get the empty bucket to remove and possibly non-empty bucket to keep
        let non_empty_bucket_index = self.get_bucket_index_merge_candidate(&directory_page, empty_bucket_index);
        assert_ne!(
            empty_bucket_index, non_empty_bucket_index,
            "Bucket index to delete cannot be the same as the bucket index to remove"
        );

        // 5. If the bucket that should be merge with does not have the same depth, do not merge
        if directory_page.get_local_depth(non_empty_bucket_index) != bucket_local_depth {
            return Ok(());
        }

        // 6. Try to fetch the bucket page to merge with
        let new_buckets_page_id = directory_page.get_bucket_page_id(non_empty_bucket_index);
        let non_empty_bucket_guard = self.bpm.fetch_page_write(new_buckets_page_id, AccessType::Unknown);
        let non_empty_bucket_guard = match non_empty_bucket_guard {
            Ok(v) => v,
            Err(error) => {
                // This is an error, but does not interrupt with the logic of the value removal, so we log it and not propagate
                eprintln!("Failed to fetch the bucket page to merge with, skipping merge\n {}", error);

                return Ok(());
            }
        };

        let empty_bucket_index_local_depth = directory_page.get_local_depth(empty_bucket_index);

        let new_bucket_local_depth = (empty_bucket_index_local_depth - 1) as u8;

        // 7. Get the first bucket index that is going to be updated
        let starting_bucket_index = empty_bucket_index.get_n_lsb_bits(new_bucket_local_depth);

        // 8. Go over all buckets that points to either buckets (empty or non empty) and update the depth and the page id
        for bucket_index_to_update in starting_bucket_index.get_all_numbers_with_prefix_bits_until_max_bits(new_bucket_local_depth, directory_page.get_global_depth() as u8) {
            directory_page.decr_local_depth(bucket_index_to_update);

            // Pointing to the possibly not empty bucket to avoid moving data around
            directory_page.set_bucket_page_id(bucket_index_to_update, new_buckets_page_id);
        }

        let empty_bucket_page_id = empty_bucket_page_guard.get_page_id();

        // 9. Drop the empty bucket so we can delete it
        //    there is nothing pointing to this page so it is safe to drop the lock so we can delete it
        drop(empty_bucket_page_guard);

        // 10. remove the empty bucket after moving everything
        self.bpm
            .delete_page(empty_bucket_page_id)
            .map_err_to_buffer_pool_err()
            .context("Was unable to delete empty bucket page, dangling page")?;

        // 11. Shrink the directory if needed
        if directory_page.can_shrink() {
            directory_page.decr_global_depth();
        }

        // 12. Check if after the merge the bucket is still empty
        if non_empty_bucket_guard.cast::<<Self as TypeAliases>::BucketPage>().is_empty() {
            // 13. Try to merge current bucket as well
            return self.trigger_merge(
                directory_page_guard,
                non_empty_bucket_guard,
                non_empty_bucket_index,
            );
        }

        Ok(())
    }

    /// Return the page indices and page guard to the page that is empty and to the page that is possibly not empty (the one to keep)
    ///
    /// # Arguments
    ///
    /// * `directory_page`:
    /// * `bucket_page_guard`:
    /// * `bucket_index`:
    ///
    /// returns: u32 Bucket index
    fn get_bucket_index_merge_candidate<'a>(&self, directory_page: &<Self as TypeAliases>::DirectoryPage, mut bucket_index: u32, ) -> u32 {
        let local_depth = directory_page.get_local_depth(bucket_index);

        // 1. Trim bucket index to the first index that point to the bucket
        bucket_index = bucket_index & directory_page.get_local_depth_mask(bucket_index);

        // 2. Get the other bucket index
        bucket_index.toggle_bit(local_depth as usize)
    }

    fn remove_directory(
        &self,
        header_page_guard: &mut PageWriteGuard,
        directory_page_guard: PageWriteGuard,
        directory_index: u32,
    ) -> Result<(), BufferPoolError> {
        header_page_guard
            .cast_mut::<<Self as TypeAliases>::HeaderPage>()
            .set_directory_page_id(directory_index, INVALID_PAGE_ID);

        let directory_page_id = directory_page_guard.get_page_id();

        // Drop the directory to be able to delete it
        drop(directory_page_guard);

        println!(
            "Trying to delete directory with page id: {}",
            directory_page_id
        );

        let deleted = self
            .bpm
            .delete_page(directory_page_id)
            .map_err_to_buffer_pool_err()
            .context("Was unable to delete empty directory page")?;

        assert_eq!(deleted, true, "Should be able to delete page");

        Ok(())
    }

    fn remove_bucket(
        &self,
        directory_page_guard: &mut PageWriteGuard,
        bucket_page_guard: PageWriteGuard,
        bucket_index: u32,
    ) -> Result<(), BufferPoolError> {
        directory_page_guard
            .cast_mut::<<Self as TypeAliases>::DirectoryPage>()
            .set_bucket_page_id(bucket_index, INVALID_PAGE_ID);

        let bucket_page_id = bucket_page_guard.get_page_id();

        // Drop the bucket to be able to delete it
        drop(bucket_page_guard);

        let deleted = self
            .bpm
            .delete_page(bucket_page_id)
            .map_err_to_buffer_pool_err()
            .context("Was unable to delete empty bucket page")?;

        assert_eq!(deleted, true, "Should be able to delete page");

        Ok(())
    }
}
