#[cfg(test)]
mod tests {
    use crate::buffer::BufferPoolManager;
    use crate::catalog::Schema;
    use crate::container::DiskExtendibleHashTable;
    use crate::storage::{hash_table_bucket_array_size, DiskManagerUnlimitedMemory, GenericComparator, GenericKey};
    use common::config::{PageId, BUSTUB_PAGE_SIZE};
    use common::RID;
    use generics::Shuffle;
    use parking_lot::Mutex;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::collections::HashSet;
    use std::sync::Arc;

    fn create_extendible_hash_table() -> DiskExtendibleHashTable<{ hash_table_bucket_array_size::<GenericKey<8>, RID>() }, GenericKey<8>, RID, GenericComparator<8>> {
        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = Arc::new(BufferPoolManager::new(4, disk_manager, Some(2), None));

        let key_schema = Schema::parse_create_statement("a bigint").expect("Should be able to create schema");

        DiskExtendibleHashTable::new(
            "temp".to_string(),
            bpm,
            GenericComparator::from(key_schema),
            None,
            None,
        )
    }

    #[test]
    fn should_allow_basic_hash_map_operation_on_a_lot_of_keys_across_multiple_pages() {
        let mut hash_table = create_extendible_hash_table();

        let total = (BUSTUB_PAGE_SIZE * 10) as i64;

        // Should not find any values before init
        for &i in &(0..total).shuffle()[0..10] {
            let mut index_key = GenericKey::<8>::default();
            index_key.set_from_integer(i);

            assert_eq!(hash_table.get_value(&index_key, None), vec![], "should not find values for key {}", i);
        }

        // insert a few (key, value) pairs
        for i in 0..total {
            let mut index_key = GenericKey::<8>::default();
            index_key.set_from_integer(i);

            let rid = RID::new(i as PageId, i as u32);

            assert!(hash_table.insert(&index_key, &rid, None), "should insert new key {}", i);
        }

        // Should not find missing keys after the hash map is initialized
        for &i in &(total..total + 1_000_000).shuffle()[0..10] {
            let mut index_key = GenericKey::<8>::default();
            index_key.set_from_integer(i);

            assert_eq!(hash_table.get_value(&index_key, None), vec![], "should not find values for key {}", i);
        }

        // Fetch those in random order
        for i in (0..total).shuffle() {
            let mut index_key = GenericKey::<8>::default();
            index_key.set_from_integer(i);

            let rid = RID::new(i as PageId, i as u32);

            assert_eq!(hash_table.get_value(&index_key, None), vec![rid], "should find values for key {}", i);
        }

        // Remove 1/7 random keys
        let random_key_index_to_remove = &(0..total).shuffle()[0..(total / 7) as usize];

        for &i in random_key_index_to_remove {
            let mut index_key = GenericKey::<8>::default();
            index_key.set_from_integer(i);

            let rid = RID::new(i as PageId, i as u32);

            assert!(hash_table.remove(&index_key, None), "should remove key {}", i);
        }

        // Fetch all in random order
        {
            let removed_keys = HashSet::<i64>::from_iter(random_key_index_to_remove.iter().cloned());

            for i in (0..total).shuffle() {
                let mut index_key = GenericKey::<8>::default();
                index_key.set_from_integer(i);

                let expected_return = if removed_keys.contains(&i) { vec![] } else { vec![RID::new(i as PageId, i as u32)] };

                assert_eq!(hash_table.get_value(&index_key, None), expected_return, "get value for key {}", i);
            }
        }

        let mut removed_random_keys_to_reinsert = random_key_index_to_remove.iter().cloned().collect::<Vec<_>>().clone();
        removed_random_keys_to_reinsert.shuffle(&mut thread_rng());

        // Add back 1/4 of the removed keys
        let removed_random_keys_to_reinsert = &removed_random_keys_to_reinsert[0..removed_random_keys_to_reinsert.len() / 4];

        let offset_for_reinserted_values = total * 100;

        {
            // insert back some of the removed keys with different values
            for &i in removed_random_keys_to_reinsert {
                let mut index_key = GenericKey::<8>::default();
                index_key.set_from_integer(i);

                let rid_value = i + offset_for_reinserted_values;
                let rid = RID::new(rid_value as PageId, rid_value as u32);

                assert!(hash_table.insert(&index_key, &rid, None), "should insert new key {}", i);
            }
        }

        {
            let reinserted_keys = HashSet::<i64>::from_iter(removed_random_keys_to_reinsert.iter().cloned());
            let removed_keys = HashSet::<i64>::from_iter(random_key_index_to_remove.iter().cloned());

            let removed_keys: HashSet::<i64> = &removed_keys - &reinserted_keys;

            // Fetch all in random order
            for i in (0..total).shuffle() {
                let mut index_key = GenericKey::<8>::default();
                index_key.set_from_integer(i);

                let found_value = hash_table.get_value(&index_key, None);

                if removed_keys.contains(&i) {
                    assert_eq!(found_value, vec![], "should not find any values for removed key {}", i);
                    continue;
                } else if reinserted_keys.contains(&i) {
                    let rid_value = i + offset_for_reinserted_values;
                    let reinserted_value = RID::new(rid_value as PageId, rid_value as u32);

                    assert_eq!(found_value, vec![reinserted_value], "should find updated value for reinserted key {}", i);
                } else {
                    let value = RID::new(i as PageId, i as u32);

                    assert_eq!(found_value, vec![value], "should find original value for not changed key {}", i);
                }
            }
        }
    }

    // TODO - add test for deletion to already deleted should not do anything
}
