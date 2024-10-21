use buffer_common::AccessType;
use buffer_pool_manager::BufferPool;
use common::{Comparator, PageKey, PageValue};
use hashing_common::KeyHasher;
use crate::bucket_page::BucketPage;
use crate::directory_page::DirectoryPage;
use crate::DiskHashTable;
use crate::header_page::HeaderPage;

pub struct HashTableIterator<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    // Holding mutable reference so the compiler will force the hash table to not change
    hash_table: &'a mut DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>,
    
}

impl<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> HashTableIterator<'a, BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher,
{
    pub(crate) fn new(hash_table: &'a mut DiskHashTable<BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>) -> Self {

        let header_page_guard = hash_table.bpm.fetch_page_read(
            hash_table.header_page_id,
            AccessType::Unknown
        ).expect("Failed to fetch header page");

        let header = header_page_guard.cast::<HeaderPage>();
        
        // let iter = header
        //     .iter()
        //     .flat_map(|directory_page_id| {
        //         let directory_page_guard = hash_table.bpm.fetch_page_read(
        //             directory_page_id,
        //             AccessType::Unknown
        //         ).expect("Failed to fetch directory page");
        //         
        //         DirectoryPage::create_iter(directory_page_guard)
        //             .flat_map(|bucket_page_id| {
        //                 let bucket_page_guard = hash_table.bpm.fetch_page_read(
        //                     bucket_page_id,
        //                     AccessType::Unknown
        //                 ).expect("Failed to fetch bucket page");
        // 
        //                 let bucket_page = bucket_page_guard.cast::<BucketPage<BUCKET_MAX_SIZE, Key, Value, KeyComparator>>();
        //                 
        //                 bucket_page.iter()
        //                     .map(|b| (d, bucket_page_guard, b))
        //             })
        //     });
        
        todo!();
        
        Self {
            hash_table
        }
    }
}

impl<'a, const BUCKET_MAX_SIZE: usize, Key, Value, KeyComparator, KeyHasherImpl> Iterator for HashTableIterator<'a, BUCKET_MAX_SIZE, Key, Value, KeyComparator, KeyHasherImpl>
where
    Key: PageKey,
    Value: PageValue,
    KeyComparator: Comparator<Key>,
    KeyHasherImpl: KeyHasher {
    type Item = (Key, Value);

    fn next(&mut self) -> Option<Self::Item> {
        let header_page_guard = self.hash_table.bpm.fetch_page_read(
            self.hash_table.header_page_id,
            AccessType::Unknown
        ).expect("Failed to fetch header page");
        
        let header = header_page_guard.cast::<HeaderPage>();
        // header.
        todo!()
    }
}