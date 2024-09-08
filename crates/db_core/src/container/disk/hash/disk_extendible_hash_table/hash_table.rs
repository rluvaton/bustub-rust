use crate::buffer::BufferPoolManager;
use crate::concurrency::Transaction;
use crate::container::hash::HashFunction;
use crate::storage::{hash_table_bucket_array_size, Comparator, ExtendibleHashTableBucketPage, ExtendibleHashTableDirectoryPage, ExtendibleHashTableHeaderPage, HASH_TABLE_DIRECTORY_MAX_DEPTH, HASH_TABLE_HEADER_MAX_DEPTH};
use common::config::{PageId, HEADER_PAGE_ID, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;


/// Implementation of extendible hash table that is backed by a buffer pool
/// manager. Non-unique keys are supported. Supports insert and delete. The
/// table grows/shrinks dynamically as buckets become full/empty.
pub struct HashTable<Key, Value, KeyComparator>
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
    bucket_max_size: u32,
    header_page_id: PageId, // TODO - do we need this?
}

impl<Key, Value, KeyComparator> HashTable<Key, Value, KeyComparator>
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
    /// - `bucket_max_size`: the max size allowed for the bucket page array
    ///
    pub fn new(name: String, bpm: Arc<BufferPoolManager>, cmp: KeyComparator, header_max_depth: Option<u32>, directory_max_depth: Option<u32>, bucket_max_size: Option<u32>) -> Self {
        Self {
            index_name: name,
            bpm,
            cmp,

            header_page_id: HEADER_PAGE_ID, // TODO - is this correct?

            header_max_depth: header_max_depth.unwrap_or(HASH_TABLE_HEADER_MAX_DEPTH),
            directory_max_depth: directory_max_depth.unwrap_or(HASH_TABLE_DIRECTORY_MAX_DEPTH),
            bucket_max_size: bucket_max_size.unwrap_or(hash_table_bucket_array_size::<Key, Value>() as u32),
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
        unimplemented!()
    }

    /// Helper function to verify the integrity of the extendible hash table's directory.
    pub fn verify_integrity(&self) {
        assert_ne!(self.header_page_id, INVALID_PAGE_ID, "header page id is invalid");
        let header_guard = self.bpm.fetch_page_basic(self.header_page_id).expect("Should fetch the header page");
        let header_guard = header_guard.read();

        let header = header_guard.cast::<ExtendibleHashTableHeaderPage>();

        // for each of the directory pages, check their integrity using directory page VerifyIntegrity
        for idx in 0..header.max_size() {
            let directory_page_id = header.get_directory_page_id(idx);
            if directory_page_id != INVALID_PAGE_ID {
                let directory_guard = self.bpm.fetch_page_basic(directory_page_id).expect("Should fetch directory page");
                let directory_guard = directory_guard.read();

                let directory = directory_guard.cast::<ExtendibleHashTableDirectoryPage>();
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
    fn hash(&self, key: Key) -> u32 {
        HashFunction::get_hash(key) as u32
    }



    fn insert_to_new_directory(&self, header: &ExtendibleHashTableHeaderPage, directory_idx: u32, hash: u32, key: &Key, value: &Value) -> bool {
        todo!()
    }

    fn update_directory_mapping(&self, header: &ExtendibleHashTableDirectoryPage, new_bucket_idx: u32, new_bucket_page_id: PageId, new_local_depth: u32, local_depth_mask: u32) -> bool {
        todo!()
    }

    fn migrate_entries<const ARRAY_SIZE: usize>(&self, old_bucket: &ExtendibleHashTableBucketPage<ARRAY_SIZE, Key, Value, KeyComparator>, new_bucket: &ExtendibleHashTableBucketPage<ARRAY_SIZE, Key, Value, KeyComparator>, new_bucket_idx: u32, local_depth_mask: u32) -> bool {
        todo!()
    }
}

impl<Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key>> Debug for HashTable<Key, Value, KeyComparator> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n================ PRINT! ================\n")?;

        let header_guard = self.bpm.fetch_page_basic(self.header_page_id).expect("Should fetch the header page");
        let header_guard = header_guard.read();

        let header = header_guard.cast::<ExtendibleHashTableHeaderPage>();

        write!(f, "{:?}", header)?;

        for idx in 0..header.max_size() {
            let directory_page_id = header.get_directory_page_id(idx);
            if directory_page_id == INVALID_PAGE_ID {
                write!(f, "Directory {}, page_id: {}\n", idx, directory_page_id)?;
                continue;
            }

            let directory_guard = self.bpm.fetch_page_basic(directory_page_id).expect("Should fetch directory page");
            let directory_guard = directory_guard.read();

            let directory = directory_guard.cast::<ExtendibleHashTableDirectoryPage>();
            write!(f, "Directory {}, page_id: {}\n", idx, directory_page_id)?;

            write!(f, "{:?}", directory)?;

            for idx2 in 0..directory.size() {
                let bucket_page_id = directory.get_bucket_page_id(idx2);
                let bucket_guard = self.bpm.fetch_page_basic(bucket_page_id).expect("Should be able to fetch bucket page");
                let bucket_guard = bucket_guard.read();

                let bucket = bucket_guard.cast::<ExtendibleHashTableBucketPage<1, Key, Value, KeyComparator>>();

                write!(f, "Bucket {}, page id: {}\n", idx2, bucket_page_id)?;
                write!(f, "{:?}", bucket)?;
            }
        }

        f.write_str("\n================ END OF PRINT! ================\n")
    }
}

