#[cfg(test)]
mod tests {
    use crate::buffer::BufferPoolManager;
    use crate::catalog::Schema;
    use crate::storage::{hash_table_bucket_array_size, DiskManagerUnlimitedMemory, ExtendibleHashTableBucketPage, GenericComparator, GenericKey};
    use common::config::PageId;
    use common::RID;
    use parking_lot::Mutex;
    use std::sync::Arc;

    #[test]
    fn bucket_page_sample() {
        let disk_mgr = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = Arc::new(BufferPoolManager::new(5, disk_mgr, None, None));

        {
            let guard = bpm.new_page_guarded().expect("Should be able to create new page");
            let mut guard = guard.upgrade_write();

            const BUCKET_SIZE: usize = hash_table_bucket_array_size::<GenericKey<8>, RID>();
            let bucket_page = guard.cast_mut::<ExtendibleHashTableBucketPage<BUCKET_SIZE, GenericKey<8>, RID, GenericComparator<8>>>();
            bucket_page.init(Some(10));

            let key_schema = Schema::parse_create_statement("a bigint").expect("Should be able to create schema");
            let comparator = GenericComparator::<8>::from(Arc::clone(&key_schema));
            let mut index_key = GenericKey::<8>::default();
            let mut rid = RID::default();

            // insert a few (key, value) pairs
            for i in 0..10 {
                index_key.set_from_integer(i);
                rid.set(i as PageId, i as u32);
                assert!(bucket_page.insert(&index_key, &rid, &comparator), "should insert new key {}", i);
            }

            index_key.set_from_integer(11);
            rid.set(11, 11);
            assert!(bucket_page.is_full(), "bucket should be full");
            assert_eq!(bucket_page.insert(&index_key, &rid, &comparator), false, "should not insert existing key");

            // check for the inserted pairs
            for i in 0..10 {
                index_key.set_from_integer(i);
                let rid_value = bucket_page.lookup(&index_key, &comparator).cloned().map(|item| item.1);
                assert_eq!(rid_value, Some(RID::new(i as PageId, i as u32)), "Should find key {} and", i)
            }

            // remove a few pairs
            for i in 0..10 {
                if i % 2 == 1 {
                    index_key.set_from_integer(i);
                    assert!(bucket_page.remove(&index_key, &comparator), "Should be able to remove {}", i);
                }
            }

            for i in 0..10 {
                if i % 2 == 1 {
                    // remove the same pairs again
                    index_key.set_from_integer(i);
                    assert_eq!(bucket_page.remove(&index_key, &comparator), false, "Should not remove already removed {}", i);
                } else {
                    index_key.set_from_integer(i);
                    assert!(bucket_page.remove(&index_key, &comparator), "Should be able to remove {}", i);
                }
            }
            assert!(bucket_page.is_empty(), "Page should be empty");
        }  // page guard dropped
    }
}
