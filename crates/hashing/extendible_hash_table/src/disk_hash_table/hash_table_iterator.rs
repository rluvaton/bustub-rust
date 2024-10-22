use crate::bucket_array_size;
use std::iter::{Flatten, Map};
use std::slice::Iter;
use buffer_common::AccessType;
use buffer_pool_manager::BufferPool;
use common::{Comparator, PageKey, PageValue};
use hashing_common::KeyHasher;
use pages::PageId;
use crate::bucket_page::{BucketPage, BucketPageIterState, MappingType};
use crate::directory_page::{DirectoryIter, DirectoryIterState, DirectoryPage};
use crate::{bucket_page_type, DiskHashTable};
use crate::header_page::{HeaderIter, HeaderIterState, HeaderPage};

pub struct HashTableIterator<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    // Holding mutable reference so the compiler will force the hash table to not change
    hash_table: &'a DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>,

    header_iterator_state: HeaderIterState,

    // The first item is the directory page id (last value of the header iterator state)
    // and the second value is the directory iterator state
    directory_iterator_state: Option<(PageId, DirectoryIterState)>,

    // The first item is the Bucket page id (last value of the directory iterator state)
    // and the second value is the bucket iterator state
    bucket_iterator_state: Option<(PageId, BucketPageIterState)>,
}

impl<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTableIterator<'a, BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    pub(crate) fn new(hash_table: &'a DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>) -> Self {
        let initial_header_state = {
            // TODO - use with header page
            let header_page_guard = hash_table.bpm.fetch_page_read(
                hash_table.header_page_id,
                AccessType::Unknown,
            ).expect("Failed to fetch header page");

            let header = header_page_guard.cast::<HeaderPage>();

            header.iter().get_state()
        };

        Self {
            hash_table,
            header_iterator_state: initial_header_state,
            directory_iterator_state: None,
            bucket_iterator_state: None,
        }
    }

    fn resume_from_bucket_iterator_state(&mut self, bucket_page_id: PageId, bucket_iterator_state: BucketPageIterState) -> Option<MappingType<Key, Value>> {
        let bucket_page_guard = self.hash_table.bpm.fetch_page_read(
            bucket_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch bucket page");

        let bucket = bucket_page_guard.cast::<BucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>>();

        let mut bucket_page_iter = bucket.resume_iter(bucket_iterator_state);

        let entry = bucket_page_iter.next();

        if let Some(entry) = entry {
            self.bucket_iterator_state.replace((bucket_page_id, bucket_page_iter.get_state()));

            return Some(entry.clone());
        }

        // Remove the bucket iterator state
        self.bucket_iterator_state.take();

        // If here, then the current bucket page iterator finished
        None
    }

    fn resume_from_directory_iterator_state(&mut self, directory_page_id: PageId, directory_iterator_state: DirectoryIterState) -> Option<(PageId, BucketPageIterState)> {
        let directory_page_guard = self.hash_table.bpm.fetch_page_read(
            directory_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch directory page");

        let directory_page = directory_page_guard.cast::<DirectoryPage>();


        let mut directory_page_iter = directory_page.resume_iter(directory_iterator_state);

        let next_bucket_page_id = directory_page_iter.next();

        if let Some(bucket_page_id) = next_bucket_page_id {
            self.directory_iterator_state.replace((directory_page_id, directory_page_iter.get_state()));

            let bucket_page_guard = self.hash_table.bpm.fetch_page_read(
                bucket_page_id,
                AccessType::Unknown,
            ).expect("Failed to fetch bucket page");

            let bucket = bucket_page_guard.cast::<BucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>>();
            let bucket_iterator_state = bucket.iter().get_state();

            self.bucket_iterator_state.replace((bucket_page_id, bucket_iterator_state));

            Some((bucket_page_id, bucket_iterator_state))
        } else {

            // Remove the directory iterator state
            self.directory_iterator_state.take();

            // If here, then the current directory page iterator finished
            None
        }
    }

    fn resume_from_header_iterator_state(&mut self, header_iterator_state: HeaderIterState) -> Option<(PageId, DirectoryIterState)> {
        let header_page_guard = self.hash_table.bpm.fetch_page_read(
            self.hash_table.header_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch header page");

        let header = header_page_guard.cast::<HeaderPage>();
        let mut header_page_iter = header.resume_iter(header_iterator_state);

        let value = header_page_iter.next();

        self.header_iterator_state = header_page_iter.get_state();

        if let Some(directory_page_id) = value {
            let directory_page_guard = self.hash_table.bpm.fetch_page_read(
                directory_page_id,
                AccessType::Unknown,
            ).expect("Failed to fetch directory page");

            let directory_page_iterator_state = directory_page_guard.cast::<DirectoryPage>().iter().get_state();

            self.directory_iterator_state.replace((directory_page_id, directory_page_iterator_state));

            Some((directory_page_id, directory_page_iterator_state))
        } else {
            None
        }
    }
}

impl<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> Iterator for HashTableIterator<'a, BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    type Item = (Key, Value);

    fn next(&mut self) -> Option<Self::Item> {
        // If we have bucket iterator stopped than we resume from there
        if let Some((bucket_page_id, bucket_iterator_state)) = self.bucket_iterator_state {
            let next_entry = self.resume_from_bucket_iterator_state(bucket_page_id, bucket_iterator_state);

            if next_entry.is_some() {
                return next_entry;
            }

            // If none, then the current bucket iterator finished, and we need to move to the next bucket
        }


        if let Some((directory_page_id, directory_iterator_state)) = self.directory_iterator_state {
            // Go over all directory buckets until finding the next bucket with value
            while let Some(next_bucket) = self.resume_from_directory_iterator_state(directory_page_id, directory_iterator_state) {
                let next_entry = self.resume_from_bucket_iterator_state(next_bucket.0, next_bucket.1);

                if next_entry.is_some() {
                    return next_entry;
                }
            }

            // If here, then the current directory page iterator finished
        }

        while let Some((directory_page_id, directory_iterator_state)) = self.resume_from_header_iterator_state(self.header_iterator_state) {
            while let Some(next_bucket) = self.resume_from_directory_iterator_state(directory_page_id, directory_iterator_state) {
                let next_entry = self.resume_from_bucket_iterator_state(next_bucket.0, next_bucket.1);

                if next_entry.is_some() {
                    return next_entry;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{bucket_array_size, DiskHashTable};
    use buffer_pool_manager::BufferPoolManager;
    use common::{Comparator, OrdComparator, PageKey, PageValue, U64Comparator};
    use disk_storage::DiskManagerUnlimitedMemory;
    use generics::Shuffle;
    use hashing_common::{DefaultKeyHasher, KeyHasher, U64IdentityKeyHasher};
    use pages::PAGE_SIZE;
    use rand::seq::SliceRandom;
    use rand::{thread_rng, Rng, SeedableRng};
    use rand_chacha::ChaChaRng;
    use std::collections::HashSet;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::sync::Barrier;
    use std::thread;
    use crate::errors::InsertionError;

    type TestKey = u32;
    type TestValue = u64;

    fn create_extendible_hash_table(pool_size: usize) -> DiskHashTable<{ bucket_array_size::<TestKey, TestValue>() }, TestKey, TestValue, OrdComparator<TestKey>, DefaultKeyHasher> {
        let bpm = BufferPoolManager::builder()
            .with_pool_size(pool_size)
            .with_disk_manager(DiskManagerUnlimitedMemory::new())
            .with_lru_k_eviction_policy(2)
            .build_arc();

        DiskHashTable::new(
            "temp".to_string(),
            bpm,
            OrdComparator::default(),
            None,
            None,
            None,
        ).expect("Should be able to create hash table")
    }

    #[test]
    fn should_get_all_entries_in_iterator() {
        let mut hash_table = create_extendible_hash_table(1000);

        // Having enough keys so a split would happen
        let total = 10;
        
        hash_table.with_header_page(|header_page| {
            header_page.verify_empty();
        });

        hash_table.verify_integrity(false);

        let mut entries = vec![];

        for i in 0..total {
            let (key, value) = (i as TestKey, (total * 10 + i) as TestValue);

            hash_table.insert(&key, &value, None).expect("Should insert");

            entries.push((key, value));
        }

        let mut found_entries = hash_table.iter().collect::<Vec<(TestKey, TestValue)>>();

        // Sort both entries so we can compare
        entries.sort();
        found_entries.sort();

        assert_eq!(entries, found_entries);
    }

    #[test]
    fn should_get_all_entries_in_iterator_large_index() {
        let mut hash_table = create_extendible_hash_table(1000);

        // Having enough keys so a split would happen
        let total = (PAGE_SIZE * 100) as i64;

        let mut rng = thread_rng();
        let one_percent = total / 100;

        hash_table.with_header_page(|header_page| {
            header_page.verify_empty();
        });

        hash_table.verify_integrity(false);

        let mut entries = vec![];

        println!("Inserting {} entries", total);

        // Retry to make sure more than 1 bucket is full
        let mut bucket_is_full_retries = 100;

        for i in 0..total {
            let (key, value) = rng.gen();

            if i % (10 * one_percent) == 0 {
                println!("Inserted {}%", i / one_percent);
            }

            let insert_result = hash_table.insert(&key, &value, None);

            match insert_result {
                Ok(_) => {
                    entries.push((key, value));
                    continue;
                }
                Err(err) => {
                    match err {
                        // We will generate a different key - TODO - reset `i` by 1 
                        InsertionError::KeyAlreadyExists => continue,

                        InsertionError::ReachedSplitRetryLimit(_) | InsertionError::BufferPoolError(_) => panic!("Unexpected error {:?}", err),
                        InsertionError::BucketIsFull => {
                            // If bucket is full
                            if bucket_is_full_retries == 0 {
                                break;
                            }

                            bucket_is_full_retries -= 1;
                        }
                    }
                }
            }
        }

        println!("All entries inserted");

        let mut found_entries = hash_table.iter().collect::<Vec<(TestKey, TestValue)>>();

        // Sort both entries so we can compare
        entries.sort();
        found_entries.sort();

        assert_eq!(entries, found_entries);
    }
}
