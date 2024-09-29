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

        // 5. Get the empty bucket to remove and possibly non-empty bucket to keep
        let ((empty_bucket_index, empty_bucket_guard), (non_empty_bucket_index, mut non_empty_bucket_guard)) = self.get_buckets_to_remove_and_keep(&directory_page, bucket_page_guard, bucket_index)?;
        assert_ne!(empty_bucket_index, non_empty_bucket_index, "Bucket index to delete cannot be the same as the bucket index to remove");

        // 6. Decrementing the buckets local depth
        directory_page.decr_local_depth(empty_bucket_index);
        directory_page.decr_local_depth(non_empty_bucket_index);

        // 7. Point both indices to the possibly non-empty bucket so we can avoid moving around data
        directory_page.set_bucket_page_id(empty_bucket_index, non_empty_bucket_guard.get_page_id());
        directory_page.set_bucket_page_id(non_empty_bucket_index, non_empty_bucket_guard.get_page_id());

        let empty_bucket_page_id = empty_bucket_guard.get_page_id();

        // 8. Drop the source bucket so we can delete it
        //    there is nothing pointing to this page so it is safe to drop the lock so we can delete it
        drop(empty_bucket_guard);

        // 10. remove the old bucket after moving everything
        self.bpm.delete_page(empty_bucket_page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty bucket page")?;

        // 11. Check if after the merge the bucket is still empty
        if non_empty_bucket_guard.cast::<<Self as TypeAliases>::BucketPage>().is_empty() {
            // 12. Try to merge current bucket as well
            return self.try_merge(directory_page_guard, non_empty_bucket_guard, non_empty_bucket_index);
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
    /// returns: Result<((u32, PinWritePageGuard), (u32, PinWritePageGuard)), MergeError> return 2 tuples, 1 tuple to remove and 1 to keep, each tuple is the bucket index and bucket page guard
    fn get_buckets_to_remove_and_keep<'a>(&mut self, directory_page: &<Self as TypeAliases>::DirectoryPage, empty_bucket_page_guard: PinWritePageGuard<'a>, mut empty_bucket_index: u32) -> Result<(
        // (bucket_index, bucket_page_guard)

        // Empty bucket to remove
        (u32, PinWritePageGuard<'a>),

        // Other bucket to keep
        (u32, PinWritePageGuard<'a>)
    ), MergeError> {
        let local_depth = directory_page.get_local_depth(empty_bucket_index);

        // 1. Trim bucket index to the first index that point to the bucket
        empty_bucket_index = empty_bucket_index & directory_page.get_local_depth_mask(empty_bucket_index);

        // 2. Get the other bucket index
        let non_empty_bucket_index = empty_bucket_index.toggle_bit(local_depth as usize);

        // 4. Get the other bucket
        let non_empty_bucket_page_id = directory_page.get_bucket_page_id(non_empty_bucket_index);

        assert_ne!(empty_bucket_page_guard.get_page_id(), non_empty_bucket_page_id, "other bucket page id can't be the same as the first bucket page id");

        let non_empty_bucket_page_guard = self.bpm.fetch_page_write(non_empty_bucket_page_id).map_err_to_buffer_pool_err().context("Failed to fetch bucket page")?;

        // 5. Return the empty bucket and the non-empty bucket
        Ok(((empty_bucket_index, empty_bucket_page_guard), (non_empty_bucket_index, non_empty_bucket_page_guard)))
    }

    fn remove_directory(&mut self, header_page_guard: &mut PinWritePageGuard, directory_page_guard: PinWritePageGuard, directory_index: u32) -> Result<(), BufferPoolError> {
        header_page_guard.cast_mut::<<Self as TypeAliases>::HeaderPage>().set_directory_page_id(directory_index, INVALID_PAGE_ID);

        let directory_page_id = directory_page_guard.get_page_id();

        // Drop the directory to be able to delete it
        drop(directory_page_guard);

        println!("Trying to delete directory with page id: {}", directory_page_id);

        let deleted = self.bpm.delete_page(directory_page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty directory page")?;

        assert_eq!(deleted, true, "Should be able to delete page");

        Ok(())

    }

    fn remove_bucket(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: PinWritePageGuard, bucket_index: u32) -> Result<(), BufferPoolError> {
        directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>().set_bucket_page_id(bucket_index, INVALID_PAGE_ID);

        let bucket_page_id = bucket_page_guard.get_page_id();

        // Drop the bucket to be able to delete it
        drop(bucket_page_guard);

        println!("Trying to delete bucket with page id: {}", bucket_page_id);

        let deleted = self.bpm.delete_page(bucket_page_id).map_err_to_buffer_pool_err().context("Was unable to delete empty bucket page")?;

        assert_eq!(deleted, true, "Should be able to delete page");

        Ok(())
    }



}


fn format_number_in_bits(n: u64, number_of_bits: u32) -> String {
    format!("{n:#064b}")[64 - number_of_bits as usize..].to_string()
}
