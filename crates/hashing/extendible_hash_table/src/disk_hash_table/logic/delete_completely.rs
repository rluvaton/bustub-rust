use super::super::type_alias_trait::TypeAliases;
use super::super::DiskHashTable;
use buffer_common::AccessType;
use buffer_pool_manager::errors::{BufferPoolError, MapErrorToBufferPoolError};
use buffer_pool_manager::BufferPool;
use common::{Comparator, PageKey, PageValue};
use error_utils::Context;
use hashing_common::KeyHasher;
use std::fmt::Debug;
use transaction::Transaction;

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl>
DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    /// Delete the hash table completely
    ///
    /// # Arguments
    ///
    /// - `transaction`: the current transaction
    ///
    /// Returns: `true` if remove succeeded, `false` otherwise
    ///
    pub fn delete_completely(
        self,
        _transaction: &Transaction,
    ) -> Result<(), BufferPoolError> {
        let header = self
            .bpm
            .fetch_page_write(self.header_page_id, AccessType::Unknown)
            .map_err_to_buffer_pool_err()
            .context("Hash Table header page must exists when trying to delete all")?;

        let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();
        header_page
            .iter()
            .map(|directory_page_id| {
                let directory = self
                    .bpm
                    .fetch_page_write(directory_page_id, AccessType::Unknown)
                    .map_err_to_buffer_pool_err()
                    .context("Hash Table header page must exists when trying to delete all");

                if directory.is_err() {
                    // TODO - print error
                    return vec![];
                }

                let directory = directory.unwrap();

                let directory_page = directory.cast::<<Self as TypeAliases>::DirectoryPage>();

                // Collect so we can drop the directory so we will be able to delete the directory page
                let mut bucket_pages = directory_page.iter().collect::<Vec<_>>();
                drop(directory);

                bucket_pages.push(directory_page_id);

                bucket_pages
            })
            .flatten()

            // Delete and consume
            .for_each(|page_id| {
                // TODO - print error on delete page and expect the page to be deleted
                let _ = self.bpm
                    .delete_page(page_id)
                    .map_err_to_buffer_pool_err()
                    .context("Trying to delete page");
            });

        // Drop header so we can delete it
        drop(header);

        self.bpm.delete_page(self.header_page_id)
            .map_err_to_buffer_pool_err()
            .context("Trying to delete header page")?;

        Ok(())
    }

}
