use super::type_alias_trait::TypeAliases;
use crate::buffer::{BufferPoolError, BufferPoolManager, PinPageGuard, PinWritePageGuard};
use crate::concurrency::Transaction;
use crate::container::hash::KeyHasher;
use crate::storage::{Comparator, HASH_TABLE_DIRECTORY_MAX_DEPTH as DIRECTORY_MAX_DEPTH, HASH_TABLE_HEADER_MAX_DEPTH as HEADER_MAX_DEPTH};
use binary_utils::{IsBitOn, ModifyBit};
use common::config::{PageId, HEADER_PAGE_ID, INVALID_PAGE_ID};
use common::{PageKey, PageValue};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;
use error_utils::Context;
use super::errors;

const NUMBER_OF_SPLIT_RETRIES: usize = 3;

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
    /// TODO - return custom result if inserted or not - NotInsertedError
    ///
    pub fn insert(&mut self, key: &Key, value: &Value, transaction: Option<Arc<Transaction>>) -> Result<(), errors::InsertionError> {
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
            let directory_guard = self.init_new_directory()
                .context("Failed to initialize new directory page while trying to insert new entry to the hash table")?;
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
            return Err(errors::InsertionError::KeyAlreadyExists);
        }

        // 13. if bucket page is full, need to split
        if bucket_page.is_full() {
            bucket = self.trigger_split(&mut directory, bucket, bucket_index, key_hash)?;
        }

        let bucket_page = bucket.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 13. try to insert the key
        // Safety: doing unwrap as it should not happen since we split
        //         and we have a lock - TODO - add a lock
        bucket_page.insert(key, value, &self.cmp).unwrap();

        Ok(())
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
    pub fn remove(&mut self, key: &Key, transaction: Option<Arc<Transaction>>) -> error_utils::anyhow::Result<bool> {
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
    pub fn get_value(&self, key: &Key, transaction: Option<Arc<Transaction>>) -> Result<Vec<Value>, errors::LookupError> {
        // TODO - use transaction
        assert!(transaction.is_none(), "transaction is not none, transactions are not supported at the moment");

        let directory_page_id: PageId;
        let bucket_page_id: PageId;

        // 1. Hash key as most probably the hash table is initialized,
        //    and we want to avoid holding the header page read guard while hashing (even though it's fast)
        let key_hash = self.hash(key);

        {
            // 2. Get the header page
            let header = self.bpm.fetch_page_read(self.header_page_id)
                .context("Failed to fetch header")?;

            let header_page = header.cast::<<Self as TypeAliases>::HeaderPage>();

            // 3. Find the directory page id where the value might be
            let directory_index = header_page.hash_to_directory_index(key_hash);
            directory_page_id = header_page.get_directory_page_id(directory_index);
        } // Drop header page guard

        // 4. If we got invalid page than the directory is missing
        if directory_page_id == INVALID_PAGE_ID {
            return Ok(vec![])
        }

        {
            // 5. Get the directory page
            let directory = self.bpm.fetch_page_read(directory_page_id)?;

            let directory_page = directory.cast::<<Self as TypeAliases>::DirectoryPage>();

            // 6. Find the bucket page id where the value might be
            let bucket_index = directory_page.hash_to_bucket_index(key_hash);
            bucket_page_id = directory_page.get_bucket_page_id(bucket_index)
        } // Release directory page guard

        // 7. If we got invalid page than the bucket is missing
        if bucket_page_id == INVALID_PAGE_ID {
            return Ok(vec![])
        }

        let found_value: Option<Value>;

        {
            // 8. Get the bucket page
            let bucket = self.bpm.fetch_page_read(bucket_page_id)?;


            let bucket_page = bucket.cast::<<Self as TypeAliases>::BucketPage>();

            // 12. Lookup the value for the key in the target bucket
            found_value = bucket_page.lookup(key, &self.cmp)
                // Clone the value before releasing the page guard as we hold reference to something that will be freed
                .cloned();
        } // Drop bucket page guard


        Ok(found_value.map_or_else(

            // In case None, return empty results
            || vec![],

            // In case found return that result
            |v| vec![v],
        ))
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
    fn hash(&self, key: &Key) -> u32 {
        KeyHasherImpl::hash_key(key) as u32
    }

    fn trigger_split<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32, key_hash: u32) -> Result<PinWritePageGuard<'a>, errors::SplitError> {
        // Try to split the bucket with 3 iteration (after that it seems like the hash function is not good, or we have a bug)
        self.try_split(directory_page_guard, bucket_page_guard, bucket_index, key_hash, NUMBER_OF_SPLIT_RETRIES)
    }

    fn try_split<'a>(&mut self, directory_page_guard: &mut PinWritePageGuard, mut bucket_page_guard: PinWritePageGuard<'a>, bucket_index: u32, key_hash: u32, tries_left: usize) -> Result<PinWritePageGuard<'a>, errors::SplitError> {
        // 1. Check if reached max tries
        if tries_left == 0 {
            eprintln!("Trying to insert key but after split the page is still full, the hash might not evenly distribute the keys");
            return Err(errors::SplitError::ReachedRetryLimit(NUMBER_OF_SPLIT_RETRIES));
        }


        let mut directory_page = directory_page_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();
        let mut bucket_page = bucket_page_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 2. Make sure need to split
        assert!(bucket_page.is_full(), "page must be full before splitting");

        // 3. Create new bucket to be the new split bucket
        let new_bucket = self.init_new_bucket().context("Failed to initialize new bucket page when trying to split bucket")?;
        let new_bucket_page_id = new_bucket.get_page_id();
        let mut new_bucket_guard = new_bucket.upgrade_write();

        // 4. Expand the directory if needed
        if directory_page.get_local_depth(bucket_index) == directory_page.get_global_depth() {
            let increase_result = directory_page.incr_global_depth();

            if !increase_result {
                return Err(errors::SplitError::DirectoryIsFull)
            }
        }

        // 5. Split bucket
        self.split_local_bucket(bucket_index, &mut directory_page, &mut bucket_page, &mut new_bucket_guard);

        // 6. Find out which bucket to insert to after the split
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        let bucket_index_to_insert = directory_page.hash_to_bucket_index(key_hash);
        let bucket_to_insert_page_id = directory_page.get_bucket_page_id(bucket_index_to_insert);
        let bucket_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_page } else { bucket_page };

        // 7. Check if still after the split we can't insert
        if bucket_to_insert.is_full() {
            let bucket_guard_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_guard } else { bucket_page_guard };

            // 7.1 Split again with the current bucket that is full (The bucket index is always the one that about to overflow)
            return self.try_split(directory_page_guard, bucket_guard_to_insert, bucket_index_to_insert, key_hash, tries_left - 1);
        }

        let bucket_guard_to_insert = if bucket_to_insert_page_id == new_bucket_page_id { new_bucket_guard } else { bucket_page_guard };

        Ok(bucket_guard_to_insert)
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
        let header_page = bpm.new_page_guarded()?;

        assert_eq!(header_page.get_page_id(), HEADER_PAGE_ID, "must be uninitialized");
        let mut page_guard = header_page.upgrade_write();

        let page = page_guard.cast_mut::<<Self as TypeAliases>::HeaderPage>();

        page.init(Some(header_max_depth));

        Ok(())
    }

    fn init_new_directory(&mut self) -> Result<PinPageGuard, BufferPoolError> {

        // TODO - do not expect that and abort instead
        let directory_page = self.bpm.new_page_guarded().context("Should be able to create page")?;

        {
            let mut directory_guard = directory_page.write();
            let directory = directory_guard.cast_mut::<<Self as TypeAliases>::DirectoryPage>();

            directory.init(Some(self.directory_max_depth));
        }

        Ok(directory_page)
    }

    fn init_new_bucket(&mut self) -> Result<PinPageGuard, BufferPoolError> {
        let bucket_page = self.bpm.new_page_guarded().context("Should be able to create page")?;

        {
            let mut bucket_guard = bucket_page.write();
            let bucket = bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

            bucket.init(Some(self.bucket_max_size));
        }

        Ok(bucket_page)
    }

    /// Return the splitted bucket indices
    fn split_local_bucket(&mut self, mut bucket_index: u32, directory_page: &mut <Self as TypeAliases>::DirectoryPage, bucket_page_to_split: &mut <Self as TypeAliases>::BucketPage, new_bucket_guard: &mut PinWritePageGuard) {

        let new_bucket_page_id = new_bucket_guard.get_page_id();
        let new_bucket_page = new_bucket_guard.cast_mut::<<Self as TypeAliases>::BucketPage>();

        // 1. Change bucket index to be the first bucket of the specific page, so it will be the index of the bucket that will be kept as is
        let new_bucket_index = bucket_index.turn_on_bit(directory_page.get_local_depth(bucket_index) as usize + 1);

        // 2. Trim bucket index to the first index that point to the bucket
        bucket_index = bucket_index & directory_page.get_local_depth_mask(bucket_index);

        assert_ne!(bucket_index, new_bucket_index, "Bucket index cannot be the same as the new bucket index");

        // 3. Register the new bucket in the directory
        directory_page.set_bucket_page_id(new_bucket_index, new_bucket_page_id);

        // 4. Update local length for both buckets
        directory_page.incr_local_depth(bucket_index);
        directory_page.incr_local_depth(new_bucket_index);

        // 5. Rehash all current bucket page content and find the correct bucket
        let (new_bucket_items, current_bucket_items): (Vec<(Key, Value)>, Vec<(Key, Value)>) = bucket_page_to_split
            .iter()
            .partition(|(key, _)| directory_page.hash_to_bucket_index(self.hash(key)) == new_bucket_index);

        // 6. set the current bucket items in the new location
        // Optimization: Only if not empty as if nothing to add to the new bucket than it means as is
        if !new_bucket_items.is_empty() {
            new_bucket_page.replace_all_entries(new_bucket_items.as_slice());
            bucket_page_to_split.replace_all_entries(current_bucket_items.as_slice());
        }
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

