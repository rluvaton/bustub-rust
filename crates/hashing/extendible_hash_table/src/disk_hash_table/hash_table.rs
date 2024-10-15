use super::errors;
use super::type_alias_trait::TypeAliases;
use crate::directory_page::DirectoryPage;

#[cfg(feature = "statistics")]
use crate::disk_hash_table::DiskHashTableStats;

#[cfg(feature = "tracing")]
use tracy_client::span;
use crate::header_page::HeaderPage;
use buffer_common::AccessType;
use buffer_pool_manager::errors::MapErrorToBufferPoolError;
use buffer_pool_manager::{BufferPool, BufferPoolManager};
use common::{Comparator, PageKey, PageValue};
use hashing_common::KeyHasher;
use pages::{PageId, INVALID_PAGE_ID};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;

pub const HEADER_PAGE_ID: PageId = 0;                                             // the header page id


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
/// use common::OrdComparator;
/// use extendible_hash_table::{bucket_array_size, DiskExtendibleHashTable};
/// use hashing_common::DefaultKeyHasher;
///
/// type Key = u64; // can also be GenericKey<8>;
/// type Value = u16; // can also be RID;
///
/// // Your table
/// type HashTable = DiskExtendibleHashTable<
///     { bucket_array_size::<Key, Value>() },
///     Key,
///     Value,
///     OrdComparator<Key>,
///     DefaultKeyHasher
/// >;
///
/// let _ = HashTable::BUCKET_ARRAY_SIZE_OK;
/// ```
///
/// When the bucket page size is not the hash_table_bucket_array_size size
/// ```compile_fail
/// use common::OrdComparator;
/// use extendible_hash_table::{bucket_array_size, DiskExtendibleHashTable};
/// use hashing_common::DefaultKeyHasher;
///
/// type Key = u64; // can also be GenericKey<8>;
/// type Value = u16; // can also be RID;
///
/// // Your table
/// type HashTable = DiskExtendibleHashTable<
///     { bucket_array_size::<Key, Value>() - 1 }, // -1 will break
///     Key,
///     Value,
///     OrdComparator<Key>,
///     DefaultKeyHasher
/// >;
///
/// let _ = HashTable::BUCKET_ARRAY_SIZE_OK;
/// ```
pub struct DiskHashTable<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    #[allow(unused)]
    pub(super) index_name: String,
    pub(super) bpm: Arc<BufferPoolManager>,
    pub(super) cmp: KeyComparator,

    pub(super) directory_max_depth: u32,
    pub(super) bucket_max_size: u32,

    pub(super) header_page_id: PageId,

    #[cfg(feature = "statistics")]
    pub(super) stats: DiskHashTableStats,

    pub(super) phantom_data: PhantomData<(Key, Value, KeyComparator, KeyHasherImpl)>,
}

unsafe impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> Sync for DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{}

impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    pub const BUCKET_ARRAY_SIZE_OK: () = <Self as TypeAliases>::BucketPage::ARRAY_SIZE_OK;

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

        let header_max_depth = header_max_depth.unwrap_or(HeaderPage::MAX_DEPTH);
        Self::init_new_header(bpm.clone(), header_max_depth)?;

        Ok(Self {
            index_name: name,
            bpm,
            cmp,

            header_page_id: HEADER_PAGE_ID,

            directory_max_depth: directory_max_depth.unwrap_or(DirectoryPage::MAX_DEPTH),
            bucket_max_size: bucket_max_size.unwrap_or(BUCKET_MAX_SIZE as u32),

            #[cfg(feature = "statistics")]
            stats: DiskHashTableStats::default(),

            phantom_data: PhantomData,
        })
    }

    /// Helper function to verify the integrity of the extendible hash table's directory.
    pub fn verify_integrity(&self, print_content_on_failure: bool) {
        assert_ne!(self.header_page_id, INVALID_PAGE_ID, "header page id is invalid");
        let header_guard = self.bpm.fetch_page_read(self.header_page_id, AccessType::Unknown).expect("Should fetch the header page");

        let header = header_guard.cast::<<Self as TypeAliases>::HeaderPage>();

        // for each of the directory pages, check their integrity using directory page VerifyIntegrity
        for idx in 0..header.max_size() {
            let directory_page_id = header.get_directory_page_id(idx);
            if directory_page_id != INVALID_PAGE_ID {
                let directory_guard = self.bpm.fetch_page_read(directory_page_id, AccessType::Unknown).expect("Should fetch directory page");

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

    fn init_new_header(bpm: Arc<BufferPoolManager>, header_max_depth: u32) -> Result<(), errors::InitError> {
        // TODO - this should be removed, we should not create on each instance and instead it should depend if the hash table exists or not
        let mut page_guard = bpm.new_page(AccessType::Unknown).map_err_to_buffer_pool_err()?;

        assert_eq!(page_guard.get_page_id(), HEADER_PAGE_ID, "must be uninitialized");

        let page = page_guard.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        page.init(Some(header_max_depth));

        Ok(())
    }

    #[cfg(feature = "statistics")]
    pub fn get_stats(&self) -> &DiskHashTableStats {
        &self.stats
    }
}

impl<const BUCKET_MAX_SIZE: usize, Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key>, KeyHasherImpl: KeyHasher> Debug for DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n================ PRINT! ================\n")?;

        let header_guard = self.bpm.fetch_page_read(self.header_page_id, AccessType::Unknown).expect("Should fetch the header page");

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

            let directory_guard = self.bpm.fetch_page_read(directory_page_id, AccessType::Unknown).expect("Should fetch directory page");

            let directory = directory_guard.cast::<<Self as TypeAliases>::DirectoryPage>();
            write!(f, "Directory {}, page_id: {}\n", idx, directory_page_id)?;

            // Extended format

            if print_buckets_inside_directory {
                directory.extended_format(f, |bucket_page_id| {
                    let bucket_guard = self.bpm.fetch_page_read(bucket_page_id, AccessType::Unknown).expect(format!("Should be able to fetch bucket page with id {}", bucket_page_id).as_str());
                    let bucket = bucket_guard.cast::<<Self as TypeAliases>::BucketPage>();

                    bucket
                        .iter()
                        .map(|(key, _)| format!("{:?}", key))
                        .collect()
                })?
            } else {
                write!(f, "{:?}", directory)?;

                for idx2 in 0..directory.size() {
                    let bucket_page_id = directory.get_bucket_page_id(idx2);
                    let bucket_guard = self.bpm.fetch_page_read(bucket_page_id, AccessType::Unknown).expect(format!("Should be able to fetch bucket page with id {}", bucket_page_id).as_str());

                    let bucket = bucket_guard.cast::<<Self as TypeAliases>::BucketPage>();

                    write!(f, "Bucket {}, page id: {}\n", idx2, bucket_page_id)?;
                    write!(f, "{:?}", bucket)?;
                }
            }
        }

        f.write_str("\n================ END OF PRINT! ================\n")
    }
}

