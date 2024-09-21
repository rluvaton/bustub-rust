use super::type_alias_trait::TypeAliases;
use crate::buffer::{BufferPoolManager, PinPageGuard, PinWritePageGuard};
use crate::concurrency::Transaction;
use crate::container::hash::KeyHasher;
use crate::storage::{Comparator, HASH_TABLE_DIRECTORY_MAX_DEPTH as DIRECTORY_MAX_DEPTH, HASH_TABLE_HEADER_MAX_DEPTH as HEADER_MAX_DEPTH};
use anyhow::{anyhow, Context};
use binary_utils::{IsBitOn, ModifyBit};
use common::config::{PageId, HEADER_PAGE_ID, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;

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
    index_name: String,
    bpm: Arc<BufferPoolManager>,
    cmp: KeyComparator,

    // hash_fn: Hasher,

    header_max_depth: u32,
    directory_max_depth: u32,
    bucket_max_size: u32,

    header_page_id: PageId,
    phantom_data: PhantomData<(Key, Value, KeyComparator, KeyHasherImpl)>,
}


impl<const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher
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
        // Validate correct generic at compile time
        let _ = <Self as TypeAliases>::BucketPage::ARRAY_SIZE_OK;

        assert_eq!(BUCKET_MAX_SIZE as u32 as usize, BUCKET_MAX_SIZE, "Bucket max size must be u32 in size");

        let header_max_depth = header_max_depth.unwrap_or(HEADER_MAX_DEPTH);
        Self::init_new_header(bpm.clone(), header_max_depth);

        Self {
            index_name: name,
            bpm,
            cmp,

            header_page_id: HEADER_PAGE_ID,

            header_max_depth,
            directory_max_depth: directory_max_depth.unwrap_or(DIRECTORY_MAX_DEPTH),
            bucket_max_size: bucket_max_size.unwrap_or(BUCKET_MAX_SIZE as u32),

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
    /// Returns: `anyhow::Result` with empty value if succeed or error if failed
    ///
    pub fn insert(&mut self, key: &Key, value: &Value, transaction: Option<Arc<Transaction>>) -> anyhow::Result<()> {
        // TODO - use transaction
        assert!(transaction.is_none(), "transaction is not none, transactions are not supported at the moment");

        // TODO - performance improvement release write latch as soon as can

        let mut directory_page_id: PageId;
        let mut bucket_page_id: PageId;

        // 1. Hash key as most probably the hash table is initialized,
        //    and we want to avoid holding the header page write guard while hashing (even though it's fast)
        let key_hash = self.hash(key);

        // 2. Get the header page
        // TODO - get the page as read and upgrade if needed as most of the time the header page exists as well as the directory page
        let mut header = self.bpm.fetch_page_write(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;

        let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        // 4. Find the directory page id where the value might be
        let directory_index = header_page.hash_to_directory_index(key_hash);
        directory_page_id = header_page.get_directory_page_id(directory_index);

        // 5. If no directory exists create it
        if directory_page_id == INVALID_PAGE_ID {
            let directory_guard = self.init_new_directory().context("Failed to initialize new directory page while trying to insert new entry to the hash table")?;
            directory_page_id = directory_guard.get_page_id();

            // 6. Register the directory in the header page
            header_page.set_directory_page_id(directory_index, directory_page_id);
        } // Drop the new directory - on purpose we don't keep it for simplicity

        // 7. Get the directory page
        // TODO - get the page as read and upgrade if needed?
        let mut directory = self.bpm.fetch_page_write(directory_page_id).context("Directory page should exists")?;

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 8. Find the bucket page id where the value might be
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        // 9. If no bucket exists create it
        if bucket_page_id == INVALID_PAGE_ID {
            let bucket_guard = self.init_new_bucket().context("Failed to initialize new bucket page while trying to insert new entry to the hash table")?;
            bucket_page_id = bucket_guard.get_page_id();

            // 10. Register the bucket in the directory page
            directory_page.set_bucket_page_id(bucket_index, bucket_page_id);
        } // Drop the new bucket - on purpose we don't keep it for simplicity

        // 11. Get the bucket page
        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).context("Failed to fetch bucket page")?;
        let bucket_page = bucket.cast::<<Self as TypeAliases>::BucketPage>();

        // 12. If the bucket already contain the data return error
        if bucket_page.lookup(&key, &self.cmp).is_some() {
            return Err(anyhow!("Key already exists"));
        }

        // 13. if bucket page is full, need to split
        if bucket_page.is_full() {
            self.trigger_split(&mut directory, &mut bucket, directory_index, bucket_index, key_hash, key, value)?;

            return Ok(())
        }


        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 13. try to insert the key
        let inserted = bucket_page.insert(key, value, &self.cmp);

        if !inserted {
            Err(anyhow!("Failed to insert the key"))
        } else {
            Ok(())
        }
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
    pub fn remove(&mut self, key: &Key, transaction: Option<Arc<Transaction>>) -> anyhow::Result<bool> {
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
            |v| vec![v],
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
        KeyHasherImpl::hash_key(key) as u32
    }

    fn trigger_split(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: &mut PinWritePageGuard, directory_index: u32, bucket_index: u32, key_hash: u32, key: &Key, value: &Value) -> anyhow::Result<()> {
        self.try_split(directory_page_guard, bucket_page_guard, directory_index, bucket_index, key_hash, key, value, 3)
    }

    fn try_split(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: &mut PinWritePageGuard, directory_index: u32, bucket_index: u32, key_hash: u32, key: &Key, value: &Value, tries_left: usize) -> anyhow::Result<()> {
        // 1. Check if reached max tries
        if tries_left == 0 {
            eprintln!("Trying to insert key but after split the page is still full, the hash might not evenly distribute the keys");
            panic!("Can't insert key");
        }

        let bucket_page_id = bucket_page_guard.get_page_id();

        let mut directory_page = directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();
        let mut bucket_page = bucket_page_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 2. Make sure need to split
        assert!(bucket_page.is_full(), "page must be full before splitting");

        // we can split bucket without needing to split directory
        let cmp = self.cmp.clone();

        // 3. Create new bucket to be the new split bucket
        let new_bucket = self.init_new_bucket().context("Failed to initialize new bucket page when trying to split bucket")?;
        let new_bucket_page_id = new_bucket.get_page_id();
        let mut new_bucket_guard = new_bucket.upgrade_write();

        if directory_page.get_local_depth(bucket_index) != directory_page.get_global_depth() {
            // 4. Split bucket
            self.split_local_bucket_page(directory_page, bucket_page, &mut new_bucket_guard, bucket_index);

        } else {

            // 4. Expand directory
            // TODO - check if need to have directory split
            self.insert_when_bucket_is_full_and_need_to_update_global_depth(key, value, &mut directory_page, &mut bucket_page, &mut new_bucket_guard, bucket_page_id)?;
        }

        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 5. Find out which bucket to insert to

        let bucket_index_to_insert = directory_page.hash_to_bucket_index(key_hash);
        let bucket_to_insert_page_id = directory_page.get_bucket_page_id(bucket_index_to_insert);
        let bucket_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_page } else { bucket_page };

        // 6. Check if still after the split we can't insert
        if bucket_to_insert.is_full() {

            // 6.1. Try again with the expected bucket to insert
            let bucket_guard_to_insert =  if bucket_to_insert_page_id == new_bucket_page_id { &mut new_bucket_guard } else { bucket_page_guard };

            return self.try_split(directory_page_guard, bucket_guard_to_insert, directory_index, bucket_index_to_insert, key_hash, key, value, tries_left - 1);
        }

        // 7. Insert key
        let inserted = bucket_to_insert.insert(key, value, &cmp);

        if !inserted {
            Err(anyhow!("Failed to insert the key"))
        } else {
            Ok(())
        }
    }

    /// Split page when local depth is smaller than global depth in the directory
    fn split_local_bucket_page(&mut self, directory_page: &mut <Self as TypeAliases>::DirectoryPage, bucket_page_to_split: &mut <Self as TypeAliases>::BucketPage, new_bucket_guard: &mut PinWritePageGuard, bucket_index: u32) {
        // 1. Assert no need to increase directory global depth
        let old_local_depth = directory_page.get_local_depth(bucket_index);
        assert_ne!(old_local_depth, directory_page.get_global_depth());

        // 2. create new bucket page
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();
        let new_bucket_page_index: u32;

        // 3. rehash all current bucket page content and find the correct bucket
        let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_split
            .iter()
            .partition(|(key, _)| self.hash(key).is_bit_on(old_local_depth as usize + 1));

        // TODO - check if there is some items that moved to new bucket

        new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
        bucket_page_to_split.replace_all_entries(current_bucket_items.as_slice());

        // 4. increase local depth for old bucket
        // TODO - is the / 2 correct?
        assert_eq!(bucket_index % 2, 0, "new bucket index should be even in order to be the new bucket pointer");
        // TODO - is this correct?
        directory_page.incr_local_depth(bucket_index / 2);

        // 5. register bucket page in directory
        directory_page.set_bucket_page_id(bucket_index, new_bucket_guard.get_page_id());

        // 6. set local depth for new bucket
        directory_page.set_local_depth(bucket_index, old_local_depth as u8 + 1);
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

    fn init_new_header(bpm: Arc<BufferPoolManager>, header_max_depth: u32) {
        // TODO - this should be removed, we should not create on each instance and instead it should depend if the hash table exists or not
        let header_page = bpm.new_page_guarded().expect("should create page");

        assert_eq!(header_page.get_page_id(), HEADER_PAGE_ID, "must be uninitialized");
        let mut page_guard = header_page.upgrade_write();

        let page = page_guard.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        page.init(Some(header_max_depth));
    }

    fn init_new_directory(&mut self) -> anyhow::Result<PinPageGuard> {

        // TODO - do not expect that and abort instead
        let directory_page = self.bpm.new_page_guarded().context("Should be able to create page")?;

        {
            let mut directory_guard = directory_page.write();
            let directory = directory_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

            directory.init(Some(self.directory_max_depth));
        }

        Ok(directory_page)
    }

    fn init_new_bucket(&mut self) -> anyhow::Result<PinPageGuard> {
        let bucket_page = self.bpm.new_page_guarded().context("Should be able to create page")?;

        {
            let mut bucket_guard = bucket_page.write();
            let bucket = bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

            bucket.init(Some(self.bucket_max_size));
        }

        Ok(bucket_page)
    }

    fn insert_algo(&mut self, key: &Key, value: &Value) -> anyhow::Result<()> {
        // 1. Find the bucket to insert
        let key_hash = self.hash(&key);

        let mut header = self.bpm.fetch_page_write(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;

        let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        let directory_index = header_page.hash_to_directory_index(key_hash);
        let directory_page_id = header_page.get_directory_page_id(directory_index);

        let mut directory = self.bpm.fetch_page_write(directory_page_id).context("Directory page should exists")?;

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        let bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).context("Failed to fetch bucket page")?;
        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        assert!(bucket_page.insert(key, value, &self.cmp), "Should insert");

        Ok(())
    }

    /// this is the insert algorithm when the directory and bucket exists and have space
    ///
    /// # Assumptions
    /// 1. Header exists
    /// 2. Directory exists
    /// 3. Bucket exists
    /// 4. Bucket has enough space
    ///
    fn insert_when_have_bucket_with_space(&mut self, key: &Key, value: &Value) -> anyhow::Result<()> {
        // 1. Hash the key
        let key_hash = self.hash(&key);

        // 2. Fetch the header with read guard (not write as we know we don't need to update it)
        let header = self.bpm.fetch_page_read(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;
        let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

        // 3. Find the directory page id for the key
        // TODO - handle missing directory
        let directory_index = header_page.hash_to_directory_index(key_hash);
        let directory_page_id = header_page.get_directory_page_id(directory_index);

        assert_ne!(directory_page_id, INVALID_PAGE_ID, "Directory page id must exists");

        // 4. Fetch the directory with read guard (not write as we know we don't need to update it)
        let directory = self.bpm.fetch_page_read(directory_page_id).context("Directory page should exists")?;
        let directory_page = directory.cast::<<Self as TypeAliases>::DirectoryPage>();

        // 5. No longer need the header page as we know we won't be going to update it
        drop(header);

        // 6. Find the bucket page id for the key
        // TODO - handle missing bucket
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        let bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        assert_ne!(bucket_page_id, INVALID_PAGE_ID, "Bucket page id must exists");

        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).context("Failed to fetch bucket page")?;
        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // TODO - handle full bucket
        assert_eq!(bucket_page.is_full(), false, "Bucket page should have enough space");

        // 7. Insert the entry to the bucket
        assert!(bucket_page.insert(key, value, &self.cmp), "Should be able to insert new entry");

        Ok(())
    }

    /// This is the insert algorithm when the directory for the key is missing
    ///
    /// # Assumptions
    /// 1. Header exists
    /// 2. Directory is missing
    /// 3. Bucket is missing
    ///
    fn insert_when_have_first_directory(&mut self, key: &Key, value: &Value) -> anyhow::Result<()> {
        // 1. Hash the key
        let key_hash = self.hash(&key);

        // 2. Fetch the header with write guard (not read as we know we need to register the new directory it)
        let mut header = self.bpm.fetch_page_write(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;
        let header_page = header.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        // 3. Find the directory index
        let directory_index = header_page.hash_to_directory_index(key_hash);
        let directory_page_id = header_page.get_directory_page_id(directory_index);

        // TODO - handle already existing directory
        assert_eq!(directory_page_id, INVALID_PAGE_ID, "Directory should be missing");

        // 4. Create new directory
        let mut directory = self.bpm.new_page_write_guarded().context("Should be able to create directory page")?;
        let directory_page_id = directory.get_page_id();

        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();


        // 5. Register the new directory page in the header
        header_page.set_directory_page_id(directory_index, directory_page_id);

        // 6. Release the header page lock as we know that we won't need to use it anymore
        //    as the directory page is empty
        drop(header);

        // 7. Find the bucket index
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        let bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        // TODO - handle already existing bucket
        assert_eq!(bucket_page_id, INVALID_PAGE_ID, "Bucket should be missing");

        // 8. Create new bucket
        let mut bucket = self.bpm.new_page_write_guarded().context("Should be able to create bucket page")?;
        let bucket_page_id = bucket.get_page_id();

        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();


        // 9. Register the bucket page
        directory_page.set_bucket_page_id(bucket_index, bucket_page_id);

        // 10. Release the directory page lock as we know that we won't need to use it anymore
        //     as the bucket page is empty
        drop(directory);

        // 11. Insert the entry to the bucket
        assert!(bucket_page.insert(key, value, &self.cmp), "Should be able to insert new entry");

        Ok(())
    }

    /// This is for when inserting
    ///
    /// # Assumptions
    /// 1. Header exists
    /// 2. Directory exists
    /// 3. Bucket exists
    /// 4. Bucket is full
    /// 5. Bucket local depth is smaller than global depth
    ///
    fn insert_when_bucket_is_full_and_no_need_to_update_global_depth(&mut self, key: &Key, value: &Value) -> anyhow::Result<()> {
        // 1. Hash the key
        let cmp_f = &self.cmp.clone();
        let key_hash = self.hash(&key);

        // 2. Fetch the header with read guard
        //    (not write as we know we don't need to update it as we know the directory global depth does not need to be updated)
        let header = self.bpm.fetch_page_read(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;
        let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

        // 3. Find the directory page id for the key
        // TODO - handle missing directory
        let directory_index = header_page.hash_to_directory_index(key_hash);
        let directory_page_id = header_page.get_directory_page_id(directory_index);

        assert_ne!(directory_page_id, INVALID_PAGE_ID, "Directory page id must exists");

        // 4. Fetch the directory with write guard (not ead as we know we need to register the split bucket)
        let mut directory = self.bpm.fetch_page_write(directory_page_id).context("Directory page should exists")?;
        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 5. No longer need the header page as we know we won't be going to update it
        drop(header);

        // 6. Find the bucket page id for the key
        // TODO - handle missing bucket
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        let bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        let mut bucket = self.bpm.fetch_page_write(bucket_page_id).context("Failed to fetch bucket page")?;
        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        assert_eq!(bucket_page.is_full(), true, "Bucket page should be full");
        assert_ne!(directory_page.get_local_depth(bucket_index), directory_page.get_global_depth(), "Bucket local depth must be smaller than directory global depth");

        // 7. Trigger split
        // If `i` > `i_j`, then more than one entry in the bucket address table points to bucket `j`.
        // Thus, the system can split bucket `j` without increasing the size of the bucket address table.
        //
        // Observe that all the entries that point to bucket `j` correspond to hash prefixes that have the same value on the leftmost `i_j` bits.
        // The system allocates a new bucket (bucket `z`), and sets `i_j` and `i_z` to the value that results from adding 1 to the original `i_j` value.
        //
        // Next, the system needs to adjust the entries in the bucket address table that previously pointed to bucket `j`.
        // (Note that with the new value for `i_j`, not all the entries correspond to hash prefixes that have the same value on the leftmost `i_j` bits.)
        //
        // The system leaves the first half of the entries as they were (pointing to bucket `j`),
        // and sets all the remaining entries to point to the newly created bucket (bucket `z`).
        //
        // Next, as in the previous case, the system rehashes each record in bucket `j`, and allocates it either to bucket `j or to the newly created bucket `z`.
        //
        // The system then reattempts the insert.
        // In the unlikely case that it again fails, it applies one of the two cases, `i = `l_j` or `i` > `l_j`, as appropriate.
        //
        // Note that, in both cases, the system needs to recompute the hash function on only the records in bucket `j`.

        // not finished
        todo!();


        // -----------------
        // TRIGGER SPLIT
        // -----------------
        let directory_global_depth = directory_page.get_global_depth();
        let bucket_local_depth = directory_page.get_local_depth(bucket_index);

        // If `i` > `i_j`, then more than one entry in the bucket address table points to bucket `j`.
        assert!(directory_global_depth > bucket_local_depth, "Bucket local depth ({}) must be smaller than directory global depth ({})", bucket_local_depth, directory_global_depth);


        // Thus, the system can split bucket `j` without increasing the size of the bucket address table.
        //
        // Observe that all the entries that point to bucket `j` correspond to hash prefixes that have the same value on the leftmost `i_j` bits.


        // The system allocates a new bucket (bucket `z`),
        let new_bucket = self.init_new_bucket().context("Should be able to create bucket page")?;
        let new_bucket_page_id = new_bucket.get_page_id();

        let mut new_bucket_guard = new_bucket.upgrade_write();
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();


        // and sets `i_j` and `i_z` to the value that results from adding 1 to the original `i_j` value.
        directory_page.set_local_depth(bucket_index, (bucket_local_depth + 1) as u8);


        //
        // Next, the system needs to adjust the entries in the bucket address table that previously pointed to bucket `j`.
        // (Note that with the new value for `i_j`, not all the entries correspond to hash prefixes that have the same value on the leftmost `i_j` bits.)
        //
        // The system leaves the first half of the entries as they were (pointing to bucket `j`),
        // and sets all the remaining entries to point to the newly created bucket (bucket `z`).
        //
        // Next, as in the previous case, the system rehashes each record in bucket `j`, and allocates it either to bucket `j or to the newly created bucket `z`.
        //
        // The system then reattempts the insert.
        // In the unlikely case that it again fails, it applies one of the two cases, `i = `l_j` or `i` > `l_j`, as appropriate.
        //
        // Note that, in both cases, the system needs to recompute the hash function on only the records in bucket `j`.


        // 5. Insert the entry to the bucket
        // TODO - handle full bucket
        assert!(bucket_page.insert(key, value, cmp_f), "Should insert");

        Ok(())
    }

    /// This is for when inserting
    ///
    /// # Assumptions
    /// 1. Header exists
    /// 2. Directory exists
    /// 3. Bucket exists
    /// 4. Bucket is full
    /// 5. Bucket local depth is equal to global depth
    /// 6. Directory has enough space
    /// 7. directory expansion is bounded to a single directory
    ///
    fn insert_when_bucket_is_full_and_need_to_update_global_depth_from_start(&mut self, key: &Key, value: &Value) -> anyhow::Result<()> {
        let cmp_f = &self.cmp.clone();

        // 1. Hash the key
        let key_hash = self.hash(&key);

        // 2. Fetch the header with read guard
        //    (not write as we know we don't need to update it as we know the directory global depth does not need to be updated)
        let header = self.bpm.fetch_page_read(self.header_page_id).context("Hash Table header page must exists when trying to insert")?;
        let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

        // 3. Find the directory page id for the key
        // TODO - handle missing directory
        let directory_index = header_page.hash_to_directory_index(key_hash);
        let directory_page_id = header_page.get_directory_page_id(directory_index);

        assert_ne!(directory_page_id, INVALID_PAGE_ID, "Directory page id must exists");

        // 4. Fetch the directory with write guard (not ead as we know we need to register the split bucket)
        let mut directory = self.bpm.fetch_page_write(directory_page_id).context("Directory page should exists")?;
        let directory_page = directory.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

        // 5. No longer need the header page as we know we won't be going to update it
        drop(header);

        // 6. Find the bucket page id for the key
        // TODO - handle missing bucket
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        let bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        let mut bucket_to_split = self.bpm.fetch_page_write(bucket_page_id).context("Failed to fetch bucket page")?;
        let bucket_page_to_split = bucket_to_split.cast_mut::<<Self as TypeAliases>::BucketPage>();

        assert_eq!(bucket_page_to_split.is_full(), true, "Bucket page should be full");
        assert_eq!(directory_page.get_local_depth(bucket_index), directory_page.get_global_depth(), "Bucket local depth must be equal to directory global depth");


        // 7. Split
        // If `i = i_j`, only one entry in the bucket address table points to bucket `j`.
        // Therefore, the system needs to increase the size of the bucket address table so that it can include pointers to the two buckets that result from splitting bucket `j`.
        //
        // It does so by considering an additional bit of the hash value. It increments the value of `i` by 1, thus doubling the size of the bucket address table.
        // It replaces each entry with two entries, both of which contain the same pointer as the original entry.
        assert_eq!(directory_page.incr_global_depth(), true, "should be able to increase directory global depth");
        let global_depth = directory_page.get_global_depth();

        // Now two entries in the bucket address table point to bucket `j`.
        //
        // The system allocates a new bucket (bucket `z`)
        let new_bucket = self.init_new_bucket().context("Should be able to create bucket page")?;
        let new_bucket_page_id = new_bucket.get_page_id();

        let mut new_bucket_guard = new_bucket.upgrade_write();
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // and sets the second entry to point to the new bucket.
        let new_bucket_index = directory_page.hash_to_bucket_index(key_hash);
        directory_page.set_bucket_page_id(new_bucket_index, new_bucket_page_id);

        // It sets `i_j` and `i_z` to `i`.
        directory_page.set_local_depth(bucket_index, global_depth as u8);
        directory_page.set_local_depth(bucket_index, global_depth as u8);


        // Next, it rehashes each record in bucket `j` and, depending on the first `i` bits (remember the system has added 1 to `i`), either keeps it in bucket `j` or allocates it to the newly created bucket.

        // 3. rehash all current bucket page content and find the correct bucket
        let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_split
            .iter()
            .partition(|(key, _)| self.hash(key).is_bit_on(global_depth as usize + 1));

        // TODO - check if there is some items that moved to new bucket

        new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
        bucket_page_to_split.replace_all_entries(current_bucket_items.as_slice());


        // 5. Insert the entry to the bucket
        // TODO - handle full bucket again
        assert!(bucket_page_to_split.insert(key, value, cmp_f), "Should insert");

        Ok(())
    }

    /// This is for when inserting
    ///
    /// # Assumptions
    /// 1. Header exists
    /// 2. Directory exists
    /// 3. Bucket exists
    /// 4. Bucket is full
    /// 5. Bucket local depth is equal to global depth
    /// 6. Directory has enough space
    /// 7. directory expansion is bounded to a single directory
    ///
    fn insert_when_bucket_is_full_and_need_to_update_global_depth(&mut self, key: &Key, value: &Value, directory_page: &mut <Self as TypeAliases>::DirectoryPage, bucket_page_to_split: &mut <Self as TypeAliases>::BucketPage, new_bucket_guard: &mut PinWritePageGuard, bucket_page_id: PageId) -> anyhow::Result<()> {
        let cmp_f = &self.cmp.clone();

        // 1. Hash the key
        let key_hash = self.hash(&key);


        // 6. Find the bucket page id for the key
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);

        assert_eq!(bucket_page_to_split.is_full(), true, "Bucket page should be full");
        assert_eq!(directory_page.get_local_depth(bucket_index), directory_page.get_global_depth(), "Bucket local depth must be equal to directory global depth");


        // 7. Split
        // If `i = i_j`, only one entry in the bucket address table points to bucket `j`.
        // Therefore, the system needs to increase the size of the bucket address table so that it can include pointers to the two buckets that result from splitting bucket `j`.
        //
        // It does so by considering an additional bit of the hash value. It increments the value of `i` by 1, thus doubling the size of the bucket address table.
        // It replaces each entry with two entries, both of which contain the same pointer as the original entry.
        assert_eq!(directory_page.incr_global_depth(), true, "should be able to increase directory global depth");
        let global_depth = directory_page.get_global_depth();

        // Now two entries in the bucket address table point to bucket `j`.
        //
        // The system allocates a new bucket (bucket `z`)
        let new_bucket_page_id = new_bucket_guard.get_page_id();
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // and sets the second entry to point to the new bucket.
        let mut new_bucket_index = directory_page.size() - 1;
        if directory_page.get_bucket_page_id(new_bucket_index) == bucket_page_id {
            new_bucket_index = directory_page.hash_to_bucket_index(key_hash);
        }



        // If after split the page still not map to the new page, (as the prefix hash is still the same)
        // register it
            directory_page.set_bucket_page_id(new_bucket_index, new_bucket_page_id);

        // It sets `i_j` and `i_z` to `i`.
        directory_page.set_local_depth(bucket_index, global_depth as u8);
        directory_page.set_local_depth(new_bucket_index, global_depth as u8);


        // Next, it rehashes each record in bucket `j` and, depending on the first `i` bits (remember the system has added 1 to `i`), either keeps it in bucket `j` or allocates it to the newly created bucket.

        // 3. rehash all current bucket page content and find the correct bucket
        let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_split
            .iter()
            .partition(|(key, _)| directory_page.hash_to_bucket_index(self.hash(key)) == new_bucket_index);

        // TODO - check if there is some items that moved to new bucket

        new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
        bucket_page_to_split.replace_all_entries(current_bucket_items.as_slice());

        Ok(())
    }



    /// Return the splitted bucket indices
    fn split_local_bucket(&mut self, key_hash: u32, directory_page: <Self as TypeAliases>::DirectoryPage) -> (u32, u32) {
        let bucket_index = directory_page.hash_to_bucket_index(key_hash);
        let local_depth = directory_page.get_local_depth(bucket_index);
        let local_depth_mask = directory_page.get_local_depth_mask(bucket_index);
        let bucket_page_id = directory_page.get_bucket_page_id(bucket_index);

        let original_bucket_index = bucket_index & local_depth_mask;

        let new_depth_mask = local_depth_mask.turn_on_bit(local_depth as usize + 1);

        todo!()
    }
}

impl<const BUCKET_MAX_SIZE: usize, Key: PageKey, Value: PageValue, KeyComparator: Comparator<Key>, KeyHasherImpl: KeyHasher> Debug for HashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl> {
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
                let bucket_guard = self.bpm.fetch_page_basic(bucket_page_id).expect(format!("Should be able to fetch bucket page with id {}", bucket_page_id).as_str());
                let bucket_guard = bucket_guard.read();

                let bucket = bucket_guard.cast::<<Self as TypeAliases>::BucketPage>();

                write!(f, "Bucket {}, page id: {}\n", idx2, bucket_page_id)?;
                write!(f, "{:?}", bucket)?;
            }
        }

        f.write_str("\n================ END OF PRINT! ================\n")
    }
}

