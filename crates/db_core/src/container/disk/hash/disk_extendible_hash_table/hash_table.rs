use super::errors;
use super::type_alias_trait::TypeAliases;
use crate::buffer::BufferPoolManager;
use crate::container::hash::KeyHasher;
use crate::storage::{Comparator, HASH_TABLE_DIRECTORY_MAX_DEPTH as DIRECTORY_MAX_DEPTH, HASH_TABLE_HEADER_MAX_DEPTH as HEADER_MAX_DEPTH};
use common::config::{PageId, HEADER_PAGE_ID, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;
use crate::buffer::errors::{BufferPoolError, MapErrorToBufferPoolError};

/// Thread safe implementation of extendible hash table that is backed by a buffer pool
/// manager. Non-unique keys are supported. Supports insert and delete. The
/// table grows/shrinks dynamically as buckets become full/empty.
///
/// # Generics
///  - `BUCKET_MAX_SIZE`: the max size allowed for the bucket page array, get the value from `hash_table_bucket_array_size`
///
/// # Examples
///
/// ```
/// use db_core::storage::{hash_table_bucket_array_size, GenericComparator, GenericKey};
/// use db_core::container::{DiskExtendibleHashTable, DefaultKeyHasher};
/// use common::RID;
///
/// const KEY_SIZE: usize = 8;
/// type Key = GenericKey<KEY_SIZE>;
/// type Value = RID;
///
/// // Your table
/// type HashTable = DiskExtendibleHashTable<
///     { hash_table_bucket_array_size::<Key, Value>() },
///     Key,
///     Value,
///     GenericComparator<KEY_SIZE>,
///     DefaultKeyHasher
/// >;
/// ```
pub struct HashTable<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    pub(super) index_name: String,
    pub(super) bpm: Arc<BufferPoolManager>,
    pub(super) cmp: KeyComparator,

    pub(super) header_max_depth: u32,
    pub(super) directory_max_depth: u32,
    pub(super) bucket_max_size: u32,

    pub(super) header_page_id: PageId,
    pub(super) phantom_data: PhantomData<(Key, Value, KeyComparator, KeyHasherImpl)>,
}

unsafe impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> Sync for HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{}

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    /// @brief Creates a new DiskExtendibleHashTable.
    ///
    /// # Arguments
    ///
    /// - `name`:
    /// - `bpm`: bpm buffer pool manager to be used
    /// - `cmp`: comparator for keys
    /// - `hash_fn`: the hash function
    /// - `header_max_depth`: the max depth allowed for the header page
    /// - `directory_max_depth`: the max depth allowed for the directory page
    /// - `bucket_max_size`: the max size allowed for the bucket page array
    ///
    pub fn new(name: String, bpm: Arc<BufferPoolManager>, cmp: KeyComparator, header_max_depth: Option<u32>, directory_max_depth: Option<u32>, bucket_max_size: Option<u32>) -> Result<Self, errors::InitError> {
        // Validate correct generic at compile time
        let _ = <Self as TypeAliases>::BucketPage::ARRAY_SIZE_OK;

        assert_eq!(BUCKET_MAX_SIZE as u32 as usize, BUCKET_MAX_SIZE, "Bucket max size must be u32 in size");

        let header_max_depth = header_max_depth.unwrap_or(HEADER_MAX_DEPTH);
        Self::init_new_header(bpm.clone(), header_max_depth)?;

        Ok(Self {
            index_name: name,
            bpm,
            cmp,

            header_page_id: HEADER_PAGE_ID,

            header_max_depth,
            directory_max_depth: directory_max_depth.unwrap_or(DIRECTORY_MAX_DEPTH),
            bucket_max_size: bucket_max_size.unwrap_or(BUCKET_MAX_SIZE as u32),

            phantom_data: PhantomData,
        })
    }

    /// Helper function to verify the integrity of the extendible hash table's directory.
    pub fn verify_integrity(&self, print_content_on_failure: bool) {
        assert_ne!(self.header_page_id, INVALID_PAGE_ID, "header page id is invalid");
        let header_guard = self.bpm.fetch_page_basic(self.header_page_id).expect("Should fetch the header page");
        let header_guard = header_guard.read();

        let header = header_guard.cast::<<Self as TypeAliases>::HeaderPage>();

        // for each of the directory pages, check their integrity using directory page VerifyIntegrity
        for idx in 0..header.max_size() {
            let directory_page_id = header.get_directory_page_id(idx);
            if directory_page_id != INVALID_PAGE_ID {
                let directory_guard = self.bpm.fetch_page_basic(directory_page_id).expect("Should fetch directory page");
                let directory_guard = directory_guard.read();

                let directory = directory_guard.cast::<<Self as TypeAliases>::DirectoryPage>();
                directory.verify_integrity(print_content_on_failure);
            }
        }
    }

    /// Helper function to expose the header page id.
    pub fn get_header_page_id(&self) -> PageId {
        self.header_page_id
    }

    /// Helper function to print out the HashTable.
    pub fn print_hash_table(&self) {
        println!("{:?}", self)
    }

    /// Hash - simple helper to downcast MurmurHash's 64-bit hash to 32-bit
    // for extendible hashing.
    pub(super) fn hash(&self, key: &Key) -> u32 {
        KeyHasherImpl::hash_key(key) as u32
    }

    fn insert_to_new_directory(&self, header: &<Self as TypeAliases>::HeaderPage, directory_idx: u32, hash: u32, key: &Key, value: &Value) -> bool {
        todo!()
    }

    fn update_directory_mapping(&self, header: &<Self as TypeAliases>::DirectoryPage, new_bucket_idx: u32, new_bucket_page_id: PageId, new_local_depth: u32, local_depth_mask: u32) -> bool {
        todo!()
    }

    fn migrate_entries(&self, old_bucket: &<Self as TypeAliases>::BucketPage, new_bucket: &<Self as TypeAliases>::BucketPage, new_bucket_idx: u32, local_depth_mask: u32) -> bool {
        todo!()
    }

    fn init_new_header(bpm: Arc<BufferPoolManager>, header_max_depth: u32) -> Result<(), errors::InitError> {
        // TODO - this should be removed, we should not create on each instance and instead it should depend if the hash table exists or not
        let header_page = bpm.new_page_guarded().map_err_to_buffer_pool_err()?;

        assert_eq!(header_page.get_page_id(), HEADER_PAGE_ID, "must be uninitialized");
        let mut page_guard = header_page.upgrade_write();

        let page = page_guard.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        page.init(Some(header_max_depth));

        Ok(())
    }
}

impl<const BUCKET_MAX_SIZE: usize, Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key>, KeyHasherImpl: KeyHasher> Debug for HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n================ PRINT! ================\n")?;

        let header_guard = self.bpm.fetch_page_basic(self.header_page_id).expect("Should fetch the header page");
        let header_guard = header_guard.read();

        let header = header_guard.cast::<<Self as TypeAliases>::HeaderPage>();

        write!(f, "{:?}", header)?;

        // TODO - have another way of changing this as it will avoid printing the values of the keys
        let print_buckets_inside_directory = self.bucket_max_size < 5;

        for idx in 0..header.max_size() {
            let directory_page_id = header.get_directory_page_id(idx);
            if directory_page_id == INVALID_PAGE_ID {
                write!(f, "Directory {}, page_id: {}\n", idx, directory_page_id)?;
                continue;
            }

            let directory_guard = self.bpm.fetch_page_basic(directory_page_id).expect("Should fetch directory page");
            let directory_guard = directory_guard.read();

            let directory = directory_guard.cast::<<Self as TypeAliases>::DirectoryPage>();
            write!(f, "Directory {}, page_id: {}\n", idx, directory_page_id)?;

            // Extended format

            if print_buckets_inside_directory {
                directory.extended_format(f, |bucket_page_id| {
                    let bucket_guard = self.bpm.fetch_page_basic(bucket_page_id).expect(format!("Should be able to fetch bucket page with id {}", bucket_page_id).as_str());
                    let bucket_guard = bucket_guard.read();
                    let bucket = bucket_guard.cast::<<Self as TypeAliases>::BucketPage>();

                    bucket
                        .iter()
                        .map(|(key, _)| format!("{}", key))
                        .collect()
                })?
            } else {
                write!(f, "{:?}", directory)?;

                for idx2 in 0..directory.size() {
                    let bucket_page_id = directory.get_bucket_page_id(idx2);
                    let bucket_guard = self.bpm.fetch_page_basic(bucket_page_id).expect(format!("Should be able to fetch bucket page with id {}", bucket_page_id).as_str());
                    let bucket_guard = bucket_guard.read();

                    let bucket = bucket_guard.cast::<<Self as TypeAliases>::BucketPage>();

                    write!(f, "Bucket {}, page id: {}\n", idx2, bucket_page_id)?;
                    write!(f, "{:?}", bucket)?;
                }
            }
        }

        f.write_str("\n================ END OF PRINT! ================\n")
    }
}

