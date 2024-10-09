#[cfg(test)]
mod tests {
    use buffer_pool_manager::{BufferPool, BufferPoolManager};
    use buffer_common::AccessType;

    use crate::storage::{hash_table_bucket_array_size, ExtendibleHashBucketPageInsertionErrors, ExtendibleHashTableBucketPage, ExtendibleHashTableDirectoryPage, ExtendibleHashTableHeaderPage, OrdComparator};
    use pages::{PageId, INVALID_PAGE_ID};
    use parking_lot::Mutex;
    use std::sync::Arc;
    use disk_storage::DiskManagerUnlimitedMemory;

    #[test]
    fn bucket_page_sample() {
        type Key = u64;
        type Value = u16;
        let disk_mgr = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = BufferPoolManager::new(5, disk_mgr, None, None);

        {
            let mut guard = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");

            const BUCKET_SIZE: usize = hash_table_bucket_array_size::<Key, Value>();
            let bucket_page = guard.cast_mut::<ExtendibleHashTableBucketPage<{
                hash_table_bucket_array_size::<Key, Value>()
            }, Key, Value, OrdComparator<Key>>>();
            bucket_page.init(Some(10));

            let comparator = OrdComparator::<Key>::default();

            // insert a few (key, value) pairs
            for i in 0..10 {
                let key: Key = i;
                let value: Value = i as Value;
                bucket_page.insert(&key, &value, &comparator).expect(format!("should insert new key {}", i).as_str());
            }

            assert!(bucket_page.is_full(), "bucket should be full");

            assert_eq!(bucket_page.insert(&11, &11, &comparator), Err(ExtendibleHashBucketPageInsertionErrors::BucketIsFull), "should not insert when bucket is full");

            // check for the inserted pairs
            for i in 0..10 {
                let key: Key = i;
                let value: Value = i as Value;
                let actual_value = bucket_page.lookup(&key, &comparator).cloned();
                assert_eq!(actual_value, Some(value), "Should find key {} and", i)
            }

            // remove a few pairs
            for i in 0..10 {
                if i % 2 == 1 {
                    let key: Key = i;
                    assert!(bucket_page.remove(&key, &comparator), "Should be able to remove {}", i);
                }
            }

            for i in 0..10 {
                if i % 2 == 1 {
                    // remove the same pairs again
                    let key: Key = i;
                    assert_eq!(bucket_page.remove(&key, &comparator), false, "Should not remove already removed {}", i);
                } else {
                    let key: Key = i;
                    assert!(bucket_page.remove(&key, &comparator), "Should be able to remove {}", i);
                }
            }
            assert!(bucket_page.is_empty(), "Page should be empty");
        }  // page guard dropped
    }

    #[test]
    fn header_directory_page_sample() {
        type Key = u64;
        type Value = u16;

        let disk_mgr = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = BufferPoolManager::new(5, disk_mgr, None, None);



        const BUCKET_SIZE: usize = hash_table_bucket_array_size::<Key, Value>();
        type BucketPageType = ExtendibleHashTableBucketPage<BUCKET_SIZE, Key, Value, OrdComparator<Key>>;

        let mut bucket_page_id_1: PageId = INVALID_PAGE_ID;
        let mut bucket_page_id_2: PageId = INVALID_PAGE_ID;
        let mut bucket_page_id_3: PageId = INVALID_PAGE_ID;
        let mut bucket_page_id_4: PageId = INVALID_PAGE_ID;

        {
            {
                /************************ HEADER PAGE TEST ************************/
                let mut header_guard = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");

                let header_page = header_guard.cast_mut::<ExtendibleHashTableHeaderPage>();
                header_page.init(Some(2));

                // Test hashes for header page
                // 00000000000000001000000000000000 - 32768
                // 01000000000000001000000000000000 - 1073774592
                // 10000000000000001000000000000000 - 2147516416
                // 11000000000000001000000000000000 - 3221258240


                // ensure we are hashing into proper bucket based on upper 2 bits
                let hashes: [u32; 4] = [32768, 1073774592, 2147516416, 3221258240];
                for i in 0..hashes.len() {
                    assert_eq!(header_page.hash_to_directory_index(hashes[i]), i as u32)
                }
                // Dropping header page guard
            }

            /************************ DIRECTORY PAGE TEST ************************/

            // Create directory
            let mut directory_guard = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");

            let directory_page = directory_guard.cast_mut::<ExtendibleHashTableDirectoryPage>();
            directory_page.init(Some(3));

            // Create bucket No. 1
            let mut bucket_guard_1 = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");
            bucket_page_id_1 = bucket_guard_1.get_page_id();

            let bucket_page_1 = bucket_guard_1.cast_mut::<BucketPageType>();
            bucket_page_1.init(Some(10));

            // Create bucket No. 2
            let mut bucket_guard_2 = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");
            bucket_page_id_2 = bucket_guard_2.get_page_id();

            let bucket_page_2 = bucket_guard_2.cast_mut::<BucketPageType>();
            bucket_page_2.init(Some(10));

            // Create bucket No. 3
            let mut bucket_guard_3 = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");
            bucket_page_id_3 = bucket_guard_3.get_page_id();

            let bucket_page_3 = bucket_guard_3.cast_mut::<BucketPageType>();
            bucket_page_3.init(Some(10));

            // Create bucket No. 4
            let mut bucket_guard_4 = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");
            bucket_page_id_4 = bucket_guard_4.get_page_id();

            let bucket_page_4 = bucket_guard_4.cast_mut::<BucketPageType>();
            bucket_page_4.init(Some(10));


            directory_page.set_bucket_page_id(0, bucket_page_id_1);


            //
            // ======== DIRECTORY (global_depth: 0) ========
            // | bucket_idx | page_id | local_depth |
            // |    0       |    2    |    0        |
            // ================ END DIRECTORY ================
            //

            directory_page.verify_integrity(true);
            assert_eq!(directory_page.size(), 1);
            assert_eq!(directory_page.get_bucket_page_id(0), bucket_page_id_1);

            // grow the directory, local depths should change!
            directory_page.set_local_depth(0, 1);
            assert_eq!(directory_page.incr_global_depth(), true);
            directory_page.set_bucket_page_id(1, bucket_page_id_2);
            directory_page.set_local_depth(1, 1);

            // ======== DIRECTORY (global_depth: 1) ========
            // | bucket_idx | page_id | local_depth |
            // |    0       |    2    |      1      |
            // |    1       |    3    |      1      |
            // ================ END DIRECTORY ================
            //
            directory_page.verify_integrity(true);
            assert_eq!(directory_page.size(), 2);
            assert_eq!(directory_page.get_bucket_page_id(0), bucket_page_id_1);
            assert_eq!(directory_page.get_bucket_page_id(1), bucket_page_id_2);

            for i in 0..100u32 {
                assert_eq!(directory_page.hash_to_bucket_index(i), i % 2);
            }

            directory_page.set_local_depth(0, 2);
            assert_eq!(directory_page.incr_global_depth(), true);
            directory_page.set_bucket_page_id(2, bucket_page_id_3);

            // ======== DIRECTORY (global_depth: 2) ========
            // | bucket_idx | page_id | local_depth |
            // |    0       |    2    |      2      |
            // |    1       |    3    |      1      |
            // |    2       |    4    |      2      |
            // |    3       |    3    |      1      |
            // ================ END DIRECTORY ================

            directory_page.verify_integrity(true);
            assert_eq!(directory_page.size(), 4);
            assert_eq!(directory_page.get_bucket_page_id(0), bucket_page_id_1);
            assert_eq!(directory_page.get_bucket_page_id(1), bucket_page_id_2);
            assert_eq!(directory_page.get_bucket_page_id(2), bucket_page_id_3);
            assert_eq!(directory_page.get_bucket_page_id(3), bucket_page_id_2);

            for i in 0..100u32 {
                assert_eq!(directory_page.hash_to_bucket_index(i), i % 4);
            }

            directory_page.set_local_depth(0, 3);
            assert_eq!(directory_page.incr_global_depth(), true);
            directory_page.set_bucket_page_id(4, bucket_page_id_4);
            assert_eq!(directory_page.get_global_depth_mask(), 7);


            // ======== DIRECTORY (global_depth: 3) ========
            // | bucket_idx | page_id | local_depth |
            // |     0      |    2    |      3      |
            // |     1      |    3    |      1      |
            // |     2      |    4    |      2      |
            // |     3      |    3    |      1      |
            // |     4      |    5    |      3      |
            // |     5      |    3    |      1      |
            // |     6      |    4    |      2      |
            // |     7      |    3    |      1      |
            // ================ END DIRECTORY ================


            directory_page.verify_integrity(true);
            assert_eq!(directory_page.size(), 8);
            assert_eq!(directory_page.get_bucket_page_id(0), bucket_page_id_1);
            assert_eq!(directory_page.get_bucket_page_id(1), bucket_page_id_2);
            assert_eq!(directory_page.get_bucket_page_id(2), bucket_page_id_3);
            assert_eq!(directory_page.get_bucket_page_id(3), bucket_page_id_2);
            assert_eq!(directory_page.get_bucket_page_id(4), bucket_page_id_4);
            assert_eq!(directory_page.get_bucket_page_id(5), bucket_page_id_2);
            assert_eq!(directory_page.get_bucket_page_id(6), bucket_page_id_3);
            assert_eq!(directory_page.get_bucket_page_id(7), bucket_page_id_2);

            for i in 0..100u32 {
                assert_eq!(directory_page.hash_to_bucket_index(i), i % 8);
            }

            // uncommenting this code line below should cause an "Assertion failed"
            // since this would be exceeding the max depth we initialized
            // directory_page->IncrGlobalDepth();

            assert_eq!(directory_page.can_shrink(), false, "should not be able to shrink the directory since we have local depth = global depth = 3");

            directory_page.set_local_depth(0, 2);
            directory_page.set_local_depth(4, 2);
            directory_page.set_bucket_page_id(0, bucket_page_id_4);

            // ======== DIRECTORY (global_depth: 3) ========
            // | bucket_idx | page_id | local_depth |
            // |      0     |    5    |      2      |
            // |      1     |    3    |      1      |
            // |      2     |    4    |      2      |
            // |      3     |    3    |      1      |
            // |      4     |    5    |      2      |
            // |      5     |    3    |      1      |
            // |      6     |    4    |      2      |
            // |      7     |    3    |      1      |
            // ================ END DIRECTORY ================

            assert_eq!(directory_page.can_shrink(), true);
            directory_page.decr_global_depth();

            // ======== DIRECTORY (global_depth: 2) ========
            // | bucket_idx | page_id | local_depth |
            // |     0      |    5    |      2      |
            // |     1      |    3    |      1      |
            // |     2      |    4    |      2      |
            // |     3      |    3    |      1      |
            // ================ END DIRECTORY ================

            directory_page.verify_integrity(true);
            assert_eq!(directory_page.size(), 4);
            assert_eq!(directory_page.can_shrink(), false);

        }  // page guard dropped
    }
}
