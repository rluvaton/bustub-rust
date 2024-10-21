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
    hash_table: &'a mut DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>,

    header_iterator_state: HeaderIterState,
    directory_iterator_state: Option<(PageId, DirectoryIterState)>,
    bucket_iterator_state: Option<(PageId, BucketPageIterState)>,
}

impl<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTableIterator<'a, BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    pub(crate) fn new(hash_table: &'a mut DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>) -> Self {
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


    fn it(&self)
    where
        Key: PageKey,
        Value: PageValue,
        KeyComparator: Comparator<Key>,
        KeyHasherImpl: KeyHasher,
    {
        let header_page_guard = self.hash_table.bpm.fetch_page_read(
            self.hash_table.header_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch header page");

        let header = header_page_guard.cast::<HeaderPage>();

        let _ = header
            .iter()
            .map(|directory| {
                let directory_page_guard = self.hash_table.bpm.fetch_page_read(directory, AccessType::Unknown).expect("Must be able to fetch directory");

                let directory = directory_page_guard.cast::<DirectoryPage>();

                directory
                    .iter()
                    .map(|bucket_page_id| {
                        let bucket_page_guard = self.hash_table.bpm.fetch_page_read(bucket_page_id, AccessType::Unknown).expect("Must be able to fetch directory");

                        let directory = directory_page_guard.cast::<bucket_page_type!(Key, Value, KeyComparator)>();

                        directory
                            .iter()
                    })
                    .flatten()
            })
            .flatten();
    }

    fn with_bucket_page<R, F: FnOnce(&Self, &BucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>) -> R>(&self, bucket_page_id: PageId, f: F) -> R {
        let bucket_page_guard = self.hash_table.bpm.fetch_page_read(
            bucket_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch bucket page");

        let bucket = bucket_page_guard.cast::<BucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>>();

        f(self, bucket)
    }

    fn with_directory_page<R, F: FnOnce(&Self, &DirectoryPage) -> R>(&self, directory_page_id: PageId, f: F) -> R {
        let directory_page_guard = self.hash_table.bpm.fetch_page_read(
            directory_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch directory page");

        let directory = directory_page_guard.cast::<DirectoryPage>();

        f(self, directory)
    }

    fn with_header_page<R, F: FnOnce(&Self, &HeaderPage) -> R>(&self, header_page_id: PageId, f: F) -> R {
        let header_page_guard = self.hash_table.bpm.fetch_page_read(
            header_page_id,
            AccessType::Unknown,
        ).expect("Failed to fetch header page");

        let header = header_page_guard.cast::<HeaderPage>();

        f(self, header)
    }

    fn resume_from_bucket_iterator_state(&mut self, bucket_page_id: PageId, bucket_iterator_state: BucketPageIterState) -> Option<MappingType<Key, Value>> {
        let bucket_page_result = self.with_bucket_page(bucket_page_id, |this, bucket_page| {
            let mut bucket_page_iter = bucket_page.resume_iter(bucket_iterator_state);

            let entry = bucket_page_iter.next();

            if let Some(entry) = entry {
                Some(((bucket_page_id, bucket_page_iter.get_state()), entry.clone()))
            } else {
                // If reached the end - bubble up
                None
            }
        });

        if let Some((bucket_iterator_state, entry)) = bucket_page_result {
            self.bucket_iterator_state.replace(bucket_iterator_state);

            return Some(entry);
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
            self.directory_iterator_state.replace((bucket_page_id, directory_page_iter.get_state()));

            let bucket_iterator_state = self.with_bucket_page(bucket_page_id, |_, bucket_page| {
                bucket_page.iter().get_state()
            });

            self.bucket_iterator_state.replace((bucket_page_id, bucket_iterator_state));

            Some((bucket_page_id, bucket_iterator_state))
        } else {

            // Remove the directory iterator state
            self.directory_iterator_state.take();

            // If here, then the current directory page iterator finished
            None
        }
    }


    fn resume_from_header_iterator_state(&mut self, header_iterator_state: HeaderIterState) -> Option<(PageId, DirectoryIterState> {
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
        
        let have_more = self.resume_from_header_iterator_state(self.header_iterator_state);
        
        if have_more.is_none() {
            // Finished
            return None;
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