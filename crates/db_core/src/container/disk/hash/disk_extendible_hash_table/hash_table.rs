use crate::buffer::BufferPoolManager;
use crate::concurrency::Transaction;
use crate::container::hash::hash_key;
use crate::storage::{
    Comparator,
    ExtendibleHashTableBucketPage,
    ExtendibleHashTableDirectoryPage,
    ExtendibleHashTableHeaderPage,
    HASH_TABLE_DIRECTORY_MAX_DEPTH as DIRECTORY_MAX_DEPTH,
    HASH_TABLE_HEADER_MAX_DEPTH as HEADER_MAX_DEPTH,
};
use common::config::{PageId, HEADER_PAGE_ID, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;
use super::type_alias_trait::TypeAliases;

/// Implementation of extendible hash table that is backed by a buffer pool
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
/// use db_core::container::DiskExtendibleHashTable;
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
///     GenericComparator<KEY_SIZE>
/// >;
/// ```
pub struct HashTable<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
{
    index_name: String,
    bpm: Arc<BufferPoolManager>,
    cmp: KeyComparator,

    // hash_fn: Hasher,

    header_max_depth: u32,
    directory_max_depth: u32,

    // use `BUCKET_MAX_SIZE`
    // bucket_max_size: u32,
    header_page_id: PageId,
    phantom_data: PhantomData<(Key, Value, KeyComparator)>,
}


impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator> HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
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
    ///
    /// # Deprecated
    ///
    /// There used to be bucket max size argument but replaced with `BUCKET_MAX_SIZE`
    /// - `bucket_max_size`: the max size allowed for the bucket page array
    ///
    pub fn new(name: String, bpm: Arc<BufferPoolManager>, cmp: KeyComparator, header_max_depth: Option<u32>, directory_max_depth: Option<u32>) -> Self {
        // Validate correct generic at compile time
        let _ = <Self as TypeAliases>::BucketPage::ARRAY_SIZE_OK;

        assert_eq!(BUCKET_MAX_SIZE as u32 as usize, BUCKET_MAX_SIZE, "Bucket max size must be u32 in size");

        Self {
            index_name: name,
            bpm,
            cmp,

            header_page_id: HEADER_PAGE_ID,

            header_max_depth: header_max_depth.unwrap_or(HEADER_MAX_DEPTH),
            directory_max_depth: directory_max_depth.unwrap_or(DIRECTORY_MAX_DEPTH),

            // replaced with `BUCKET_MAX_SIZE`
            // bucket_max_size: bucket_max_size.unwrap_or(hash_table_bucket_array_size::<Key, Value>() as u32),
            phantom_data: PhantomData,
        }
    }

    /// TODO(P2): Add implementation
    /// Inserts a key-value pair into the hash table.
    ///
    /// # Arguments
    ///
    /// - `key`: the key to create
    /// - `value`: the value to be associated with the key
    /// - `transaction`: the current transaction
    ///
    /// Returns: `true` if insert succeeded, `false` otherwise
    ///
    pub fn insert(&mut self, key: &Key, value: &Value, transaction: Option<Arc<Transaction>>) -> bool {

        // TODO - if header is missing create it

        let key_hash = self.hash(key);

        unimplemented!()
    }

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
    pub fn remove(&mut self, key: &Key, transaction: Option<Arc<Transaction>>) -> bool {
        unimplemented!()
    }


    /// TODO(P2): Add implementation
    /// Get the value associated with a given key in the hash table.
    ///
    /// Note(fall2023): This semester you will only need to support unique key-value pairs.
    ///
    /// # Arguments
    ///
    /// - `key`: the key to look up
    /// - `transaction`: the current transaction
    ///
    /// Returns: `Vec<Value` the value(s) associated with the given key
    ///
    pub fn get_value(&self, key: &Key, transaction: Option<Arc<Transaction>>) -> Vec<Value> {
        // TODO - use transaction
        assert!(transaction.is_none(), "transaction is not none, transactions are not supported at the moment");

        let directory_page_id: PageId;
        let bucket_page_id: PageId;

        // 1. Hash key as most probably the hash table is initialized,
        //    and we want to avoid holding the header page read guard while hashing (even though it's fast)
        let key_hash = self.hash(key);

        {
            // 2. Get the header page
            let header = self.bpm.fetch_page_read(self.header_page_id);

            // 3. If header is missing than the table is not yet initialized so return empty vector
            if header.is_none() {
                // TODO - init header page if uninitialized to avoid cache hits until first set value
                return vec![];
            }
            let header = header.unwrap();

            let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

            // 4. Find the directory page id where the value might be
            let directory_index = header_page.hash_to_directory_index(key_hash);
            directory_page_id = header_page.get_directory_page_id(directory_index);
        } // Drop header page guard

        // 5. Making sure we got a valid page id
        if directory_page_id == INVALID_PAGE_ID {
            // TODO - should we panic as it's not possible?
            return vec![];
        }

        {
            // 6. Get the directory page
            let directory = self.bpm.fetch_page_read(directory_page_id);

            // 7. If directory page is missing than not found
            if directory.is_none() {
                // TODO - Should we log warning that the page don't exists?
                return vec![];
            }
            let directory = directory.unwrap();

            let directory_page = directory.cast::<<Self as TypeAliases>::DirectoryPage>();

            // 8. Find the bucket page id where the value might be
            let bucket_index = directory_page.hash_to_bucket_index(key_hash);
            bucket_page_id = directory_page.get_bucket_page_id(bucket_index)
        } // Release directory page guard

        // 9. Making sure we got a valid page id
        if bucket_page_id == INVALID_PAGE_ID {
            // TODO - should we panic as it's not possible? - or
            return vec![];
        }

        let found_value: Option<Value>;

        {
            // 10. Get the bucket page
            let bucket = self.bpm.fetch_page_read(bucket_page_id);

            // 11. If bucket page is missing than not found
            if bucket.is_none() {
                // TODO - Should we log warning that the page don't exists?
                return vec![];
            }
            let bucket = bucket.unwrap();


            let bucket_page = bucket.cast::<<Self as TypeAliases>::BucketPage>();

            // 12. Lookup the value for the key in the target bucket
            found_value = bucket_page.lookup(key, &self.cmp)
                // Clone the value before releasing the page guard as we hold reference to something that will be freed
                .cloned();
        } // Drop bucket page guard


        found_value.map_or_else(

            // In case None, return empty results
            || vec![],

            // In case found return that result
            |v| vec![v]
        )
    }

    /// Helper function to verify the integrity of the extendible hash table's directory.
    pub fn verify_integrity(&self) {
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
                directory.verify_integrity();
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
    fn hash(&self, key: &Key) -> u32 {
        hash_key(key) as u32
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
}

impl<const BUCKET_MAX_SIZE: usize, Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key>> Debug for HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n================ PRINT! ================\n")?;

        let header_guard = self.bpm.fetch_page_basic(self.header_page_id).expect("Should fetch the header page");
        let header_guard = header_guard.read();

        let header = header_guard.cast::<<Self as TypeAliases>::HeaderPage>();

        write!(f, "{:?}", header)?;

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

            write!(f, "{:?}", directory)?;

            for idx2 in 0..directory.size() {
                let bucket_page_id = directory.get_bucket_page_id(idx2);
                let bucket_guard = self.bpm.fetch_page_basic(bucket_page_id).expect("Should be able to fetch bucket page");
                let bucket_guard = bucket_guard.read();

                let bucket = bucket_guard.cast::<<Self as TypeAliases>::BucketPage>();

                write!(f, "Bucket {}, page id: {}\n", idx2, bucket_page_id)?;
                write!(f, "{:?}", bucket)?;
            }
        }

        f.write_str("\n================ END OF PRINT! ================\n")
    }
}

