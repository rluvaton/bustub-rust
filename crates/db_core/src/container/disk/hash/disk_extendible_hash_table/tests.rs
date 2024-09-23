#[cfg(test)]
mod tests {
    use crate::buffer::BufferPoolManager;
    use crate::catalog::Schema;
    use crate::container::{DefaultKeyHasher, DiskExtendibleHashTable};
    use crate::storage::{hash_table_bucket_array_size, DiskManagerUnlimitedMemory, GenericComparator, GenericKey};
    use common::config::{PageId, BUSTUB_PAGE_SIZE};
    use common::{PageKey, PageValue, RID};
    use generics::Shuffle;
    use parking_lot::Mutex;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::collections::HashSet;
    use std::sync::Arc;
    use crate::container::test_util::U64IdentityKeyHasher;
    use crate::storage::test_util::U64Comparator;

    fn create_extendible_hash_table() -> DiskExtendibleHashTable<{ hash_table_bucket_array_size::<GenericKey<8>, RID>() }, GenericKey<8>, RID, GenericComparator<8>, DefaultKeyHasher> {
        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = Arc::new(BufferPoolManager::new(4, disk_manager, Some(2), None));

        let key_schema = Schema::parse_create_statement("a bigint").expect("Should be able to create schema");

        DiskExtendibleHashTable::new(
            "temp".to_string(),
            bpm,
            GenericComparator::from(key_schema),
            None,
            None,
            None,
        )
    }

    #[test]
    fn should_allow_basic_hash_map_operation_on_a_lot_of_keys_across_multiple_pages_for_real_world_entries() {
        let mut hash_table = create_extendible_hash_table();

        // Having enough keys so a split would happen
        let total = (BUSTUB_PAGE_SIZE * 100) as i64;

        let mut index_key = GenericKey::<8>::default();
        let mut rid = RID::default();

        // Should not find any values before init
        let tmp = (0..total).shuffle();
        for &i in &tmp[0..10] {
            index_key.set_from_integer(i);

            assert_eq!(hash_table.get_value(&index_key, None), vec![], "should not find values for key {}", i);
        }

        hash_table.verify_integrity();

        // insert a few (key, value) pairs
        for i in 0..total {
            index_key.set_from_integer(i);

            rid.set(i as PageId, i as u32);

            assert!(hash_table.insert(&index_key, &rid, None).is_ok(), "should insert new key {}", i);
        }

        hash_table.verify_integrity();

        // Should not find missing keys after the hash map is initialized
        for &i in &(total..total + 1_000_000).shuffle()[0..10] {
            index_key.set_from_integer(i);

            assert_eq!(hash_table.get_value(&index_key, None), vec![], "should not find values for key {}", i);
        }

        hash_table.verify_integrity();

        // Fetch those in random order
        for i in (0..total).shuffle() {
            index_key.set_from_integer(i);

            rid.set(i as PageId, i as u32);

            assert_eq!(hash_table.get_value(&index_key, None), vec![rid], "should find values for key {}", i);
        }

        hash_table.verify_integrity();

        // Remove 1/7 random keys
        let random_key_index_to_remove = &(0..total).shuffle()[0..(total / 7) as usize];

        for &i in random_key_index_to_remove {
            index_key.set_from_integer(i);
            rid.set(i as PageId, i as u32);

            assert_eq!(hash_table.remove(&index_key, None).expect("should remove"), true, "should remove key {}", i);
        }

        hash_table.verify_integrity();

        // Fetch all in random order
        {
            let removed_keys = HashSet::<i64>::from_iter(random_key_index_to_remove.iter().cloned());

            for i in (0..total).shuffle() {
                index_key.set_from_integer(i);

                rid.set(i as PageId, i as u32);

                let expected_return = if removed_keys.contains(&i) { vec![] } else { vec![rid] };

                assert_eq!(hash_table.get_value(&index_key, None), expected_return, "get value for key {}", i);
            }
        }

        hash_table.verify_integrity();

        let mut removed_random_keys_to_reinsert = random_key_index_to_remove.iter().cloned().collect::<Vec<_>>().clone();
        removed_random_keys_to_reinsert.shuffle(&mut thread_rng());

        // Add back 1/4 of the removed keys
        let removed_random_keys_to_reinsert = &removed_random_keys_to_reinsert[0..removed_random_keys_to_reinsert.len() / 4];

        let offset_for_reinserted_values = total * 100;

        {
            // insert back some of the removed keys with different values
            for &i in removed_random_keys_to_reinsert {
                index_key.set_from_integer(i);

                let rid_value = i + offset_for_reinserted_values;
                rid.set(rid_value as PageId, rid_value as u32);

                assert!(hash_table.insert(&index_key, &rid, None).is_ok(), "should insert new key {}", i);
            }
        }

        hash_table.verify_integrity();

        {
            let reinserted_keys = HashSet::<i64>::from_iter(removed_random_keys_to_reinsert.iter().cloned());
            let removed_keys = HashSet::<i64>::from_iter(random_key_index_to_remove.iter().cloned());

            let removed_keys: HashSet::<i64> = &removed_keys - &reinserted_keys;

            // Fetch all in random order
            for i in (0..total).shuffle() {
                index_key.set_from_integer(i);

                let found_value = hash_table.get_value(&index_key, None);

                if removed_keys.contains(&i) {
                    assert_eq!(found_value, vec![], "should not find any values for removed key {}", i);
                    continue;
                } else if reinserted_keys.contains(&i) {
                    let rid_value = i + offset_for_reinserted_values;
                    rid.set(rid_value as PageId, rid_value as u32);

                    assert_eq!(found_value, vec![rid], "should find updated value for reinserted key {}", i);
                } else {
                    rid.set(i as PageId, i as u32);

                    assert_eq!(found_value, vec![rid], "should find original value for not changed key {}", i);
                }
            }
        }

        hash_table.verify_integrity();
    }

    #[test]
    fn should_work_with_split() {
        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = Arc::new(BufferPoolManager::new(4, disk_manager, Some(2), None));

        let key_schema = Schema::parse_create_statement("a bigint").expect("Should be able to create schema");

        let mut hash_table = DiskExtendibleHashTable::<{ hash_table_bucket_array_size::<GenericKey<8>, RID>() }, GenericKey<8>, RID, GenericComparator<8>, DefaultKeyHasher>::new(
            "temp".to_string(),
            bpm,
            GenericComparator::from(key_schema),
            Some(3),
            Some(3),
            Some(3),
        );


        let mut index_key = GenericKey::<8>::default();
        let mut rid = RID::default();

        let mut i = 0;

        hash_table.print_hash_table();
        hash_table.verify_integrity();

        // empty

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();

        i += 1;
        index_key.set_from_integer(i);
        rid.set(i as PageId, i as u32);
        hash_table.insert(&index_key, &rid, None).expect(format!("Should insert {}", i).as_str());

        hash_table.print_hash_table();
        println!("\n\n");
        hash_table.verify_integrity();
    }

    #[test]
    fn lifecycle_in_single_directory() {
        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = Arc::new(BufferPoolManager::new(100, disk_manager, Some(100), None));

        type Key = u64;
        type Value = u64;

        let mut hash_table = DiskExtendibleHashTable::<
            { hash_table_bucket_array_size::<Key, Value>() },
            Key,
            Value,
            U64Comparator,
            U64IdentityKeyHasher,
        >::new(
            "temp".to_string(),
            bpm,
            U64Comparator,

            // 1 directory is enough for us 2^0
            Some(0),

            // Entire entry
            None,

            // can hold up to 3 items
            Some(3),
        );

        hash_table.verify_integrity();

        // Example of value taken from
        // https://www.youtube.com/watch?v=TtkN2xRAgv4


        // Reach the initial state:
        // ```plain
        //                                   Buckets:
        //
        //                                   Local depth: 2
        //                                  ┌───┬───┬───┐
        //     Directory:          ┌───────►│ 4 │24 │16 │   1
        //                         │        └───┴───┴───┘
        //     Global depth: 2     │         Local depth: 2
        //    ┌────────────────┐1  │        ┌───┬───┬───┐
        // 00 │                ├───┘    ┌──►│   │   │   │   2
        //    ├────────────────┤2       │   └───┴───┴───┘
        // 01 │                ├────────┘    Local depth: 2
        //    ├────────────────┤3           ┌───┬───┬───┐
        // 10 │                ├───────────►│ 6 │22 │10 │   3
        //    ├────────────────┤4           └───┴───┴───┘
        // 11 │                ├────┐        Local depth: 2
        //    └────────────────┘    │       ┌───┬───┬───┐
        //                          └──────►│ 7 │31 │   │   4
        //                                  └───┴───┴───┘
        // ```
        // [AsciiFlow](https://asciiflow.com/#/share/eJyrVspLzE1VslLKzMssyUzMUSguSSxJVdJRykmsTC0CilfHKJWlFhVn5ufFKFkZ6cQoVQBpSzNTIKsSyDI2NgOySlIrSoCcGCUFgsCpNDk7taTYCsaPickjrAkdEKXJJz8Z6J%2BU1IKSDCsFI%2BI0PZrS82hKAwytwcGegOQGl8yi1OSS%2FKJKK1ymIKFpux5NaVIwASpoMgKThmYgEqjFEI%2FrICqgzClIBm7BwZ6B5D73nPwklGBANRBHKOH0AT40wRDNraSEpYEBirPgZsxB9xfCWEhowuxEJo1gvphDui%2BWGKEEO2khbmBI2BdoaAa%2BWCDH%2FcZkp2dDIuIAB4LEBTg1GxmBSLhZxpT4xQTFHaTEhCFxMTGBUD6YQrqrZ6DmMdLLFKwAu1Mg4W4Oss4Y5mMIaUJceUd8mCrVKtUCAADbshE%3D))

        for key in vec![4, 24, 16, 6, 22, 10, 7, 31] {
            hash_table.insert(&key, &key, None).unwrap();
            hash_table.verify_integrity();
        }

        // Insert 9 to the 2nd bucket
        //
        // ```plain
        //                                   Buckets:
        //
        //                                   Local depth: 2
        //                                  ┌───┬───┬───┐
        //     Directory:          ┌───────►│ 4 │24 │16 │   1
        //                         │        └───┴───┴───┘
        //     Global depth: 2     │         Local depth: 2
        //    ┌────────────────┐1  │        ┌───┬───┬───┐
        // 00 │                ├───┘    ┌──►│ 9 │   │   │   2
        //    ├────────────────┤2       │   └───┴───┴───┘
        // 01 │                ├────────┘    Local depth: 2
        //    ├────────────────┤3           ┌───┬───┬───┐
        // 10 │                ├───────────►│ 6 │22 │10 │   3
        //    ├────────────────┤4           └───┴───┴───┘
        // 11 │                ├────┐        Local depth: 2
        //    └────────────────┘    │       ┌───┬───┬───┐
        //                          └──────►│ 7 │31 │   │   4
        //                                  └───┴───┴───┘
        // ```
        hash_table.insert(&9, &9, None).unwrap();
        hash_table.verify_integrity();

        // Try to insert 20 and cause an overflow which will trigger directory expansion
        //
        // The directory expansion step:
        // ```plain
        //                                    Buckets:
        //
        //                                    Local depth: 3
        //                                   ┌───┬───┬───┐
        //      Directory:          ┌───────►│24 │16 │   │     1
        //                          │        └───┴───┴───┘
        //      Global depth: 3     │         Local depth: 2
        //     ┌────────────────┐1  │        ┌───┬───┬───┐
        // 000 │                ├───┘    ┌──►│ 9 │   │   │     2
        //     ├────────────────┤2       │   └───┴───┴───┘
        // 001 │                ├────────┤    Local depth: 2
        //     ├────────────────┤3       │   ┌───┬───┬───┐
        // 010 │                ├─┬──────┼──►│ 6 │22 │10 │     3
        //     ├────────────────┤4│      │   └───┴───┴───┘
        // 011 │                ├─┼──────┼┐   Local depth: 2
        //     ├────────────────┤5│      ││  ┌───┬───┬───┐
        // 100 │                ├─┼───┐  │├─►│ 7 │31 │   │     4
        //     ├────────────────┤2│   │  ││  └───┴───┴───┘
        // 101 │                ├─┼───┼──┘│   Local depth: 3
        //     ├────────────────┤3│   │   │  ┌───┬───┬───┐
        // 110 │                ├─┘   └───┼─►│ 4 │   │   │     5
        //     ├────────────────┤4        │  └───┴───┴───┘
        // 111 │                ├─────────┘
        //     └────────────────┘
        // ```
        // [AsciFlow](https://asciiflow.com/#/share/eJytVb1ugzAQfhXkOQM2hCqMVaUufQQWSi01KoWKkCooilQxd8iAKgbGjhk7VTwNT1LjCLD5MRhqneBAvu%2Fuu88HR%2BDZrxiY3t51V8C1IxwAExwt8I6D3db3LGCilQUO5L4xdOJFxNM0g3ghPoTkwQLK%2BLrdOy843JnNG8vyJsR117S4B9%2BxXeUJv4XPpqJNjiuSzyL5qOwy4J%2F5Su62AXZCP4jMISDGvn6LJEY62RBDo7zSzTENgqIaq03UTRjMnwE%2F5au8d%2F3HVktYTL5jiIkcpCKyM2xVLNdXVVW54mqYrE2wQaadVTZcTysMVHPJ5Ll8I04C2e6rKhzn0k45osgcFlqLhaQiUKjIpS9lzilDTztC5ZXB0hZx0uuSZmoDhdrkfUnza1v%2BWZ01y4T6sgpB8czkfBDNkjXq3JRvtKobFY6%2BbG4YsIaVnEJQPD0sq9pPrwGD%2F4CZ89P5sEgrJJ6htHOA80YfXemkJ2u9bHqaCuYoI56dIUvbARWe0qE%2F0foRGWxwAqc%2FjLIRbQ%3D%3D))
        //
        // The insertion:
        // ```plain
        //                                    Buckets:
        //
        //                                    Local depth: 3
        //                                   ┌───┬───┬───┐
        //      Directory:          ┌───────►│24 │16 │   │     1
        //                          │        └───┴───┴───┘
        //      Global depth: 3     │         Local depth: 2
        //     ┌────────────────┐1  │        ┌───┬───┬───┐
        // 000 │                ├───┘    ┌──►│ 9 │   │   │     2
        //     ├────────────────┤2       │   └───┴───┴───┘
        // 001 │                ├────────┤    Local depth: 2
        //     ├────────────────┤3       │   ┌───┬───┬───┐
        // 010 │                ├─┬──────┼──►│ 6 │22 │10 │     3
        //     ├────────────────┤4│      │   └───┴───┴───┘
        // 011 │                ├─┼──────┼┐   Local depth: 2
        //     ├────────────────┤5│      ││  ┌───┬───┬───┐
        // 100 │                ├─┼───┐  │├─►│ 7 │31 │   │     4
        //     ├────────────────┤2│   │  ││  └───┴───┴───┘
        // 101 │                ├─┼───┼──┘│   Local depth: 3
        //     ├────────────────┤3│   │   │  ┌───┬───┬───┐
        // 110 │                ├─┘   └───┼─►│ 4 │20 │   │     5
        //     ├────────────────┤4        │  └───┴───┴───┘
        // 111 │                ├─────────┘
        //     └────────────────┘
        // ```
        hash_table.insert(&20, &20, None).unwrap();
        hash_table.verify_integrity();

        // Try to insert 26 and cause an overflow which will trigger **local** bucket 3 to split
        //
        // The local bucket split:
        // ```plain
        //                                    Buckets:
        //
        //                                    Local depth: 3
        //                                   ┌───┬───┬───┐
        //      Directory:          ┌───────►│24 │16 │   │     1
        //                          │        └───┴───┴───┘
        //      Global depth: 3     │         Local depth: 2
        //     ┌────────────────┐1  │        ┌───┬───┬───┐
        // 000 │                ├───┘    ┌──►│ 9 │   │   │     2
        //     ├────────────────┤2       │   └───┴───┴───┘
        // 001 │                ├────────┤    Local depth: 3
        //     ├────────────────┤3       │   ┌───┬───┬───┐
        // 010 │                ├────────┼──►│10 │   │   │     3
        //     ├────────────────┤4       │   └───┴───┴───┘
        // 011 │                ├────────┼┐   Local depth: 2
        //     ├────────────────┤5       ││  ┌───┬───┬───┐
        // 100 │                ├─────┐  │├─►│ 7 │31 │   │     4
        //     ├────────────────┤2    │  ││  └───┴───┴───┘
        // 101 │                ├─────┼──┘│   Local depth: 3
        //     ├────────────────┤6    │   │  ┌───┬───┬───┐
        // 110 │                ├──┐  └───┼─►│ 4 │20 │   │     5
        //     ├────────────────┤4 │      │  └───┴───┴───┘
        // 111 │                ├──┼──────┘   Local depth: 3
        //     └────────────────┘  │         ┌───┬───┬───┐
        //                         └────────►│ 6 │22 │   │     6
        //                                   └───┴───┴───┘
        // ```
        // [AsciFlow](https://asciiflow.com/#/share/eJytVjFugzAUvYrlOQM2hCqMVaUuPQILpZYalUJFSJUoilQxd2BAVQbGjh07VZwmJykxxdhOABtqWfBB%2Ft%2F%2FvceT2cHQeybQCddBMIOBtyUxdODOha8kXi2j0IUOnrlwU90XtlVF2yoyTbuKErJJqgcXguFxvfafSLJy2jeuGyrknQ%2B1vLvI9wLwQF6SRweYynnH%2FP2YvzXzqyPOxE5uljHxkyjeOl2FuPnxc8xTbFULUmSfrnRxSpNQX4%2FNIhrmXM3vjvggdnkbRPcSJXxNkTHMZXZC6ZsZkjrW49UwDKE5VqaQAbaVKbNgIXDa1MAMS6GP5RMLEuiybxhoGIu85Zki%2FDc8DoUpodBUBCkoIs2SV4bli8qYkzBZ05RB%2BsqUNSk9bhmDY97ioA3pqoNU%2FMKS6C4FUwZcnd6YDRdNHWu6Z%2F6wNJj01EHqvilZdp3wz86x%2BU9sjDqD3slkespWHXpWYNE9AMwn%2Bob1M06bQeeUl%2FY99GuTa8KoK%2FJ9jDq%2FL4y%2BXmpV6NmNsaSKrfiXocM23MP9L06VbGw%3D))
        //
        // The insertion:
        // ```plain
        //                                    Buckets:
        //
        //                                    Local depth: 3
        //                                   ┌───┬───┬───┐
        //      Directory:          ┌───────►│24 │16 │   │     1
        //                          │        └───┴───┴───┘
        //      Global depth: 3     │         Local depth: 2
        //     ┌────────────────┐1  │        ┌───┬───┬───┐
        // 000 │                ├───┘    ┌──►│ 9 │   │   │     2
        //     ├────────────────┤2       │   └───┴───┴───┘
        // 001 │                ├────────┤    Local depth: 3
        //     ├────────────────┤3       │   ┌───┬───┬───┐
        // 010 │                ├────────┼──►│10 │26 │   │     3
        //     ├────────────────┤4       │   └───┴───┴───┘
        // 011 │                ├────────┼┐   Local depth: 2
        //     ├────────────────┤5       ││  ┌───┬───┬───┐
        // 100 │                ├─────┐  │├─►│ 7 │31 │   │     4
        //     ├────────────────┤2    │  ││  └───┴───┴───┘
        // 101 │                ├─────┼──┘│   Local depth: 3
        //     ├────────────────┤6    │   │  ┌───┬───┬───┐
        // 110 │                ├──┐  └───┼─►│ 4 │20 │   │     5
        //     ├────────────────┤4 │      │  └───┴───┴───┘
        // 111 │                ├──┼──────┘   Local depth: 3
        //     └────────────────┘  │         ┌───┬───┬───┐
        //                         └────────►│ 6 │22 │   │     6
        //                                   └───┴───┴───┘
        // ```
        hash_table.insert(&26, &26, None).unwrap();
        hash_table.verify_integrity();

        let all_keys = [24, 16, 9, 10, 26, 7, 31, 4, 20, 6, 22];

        for key in all_keys {
            assert_eq!(hash_table.get_value(&key, None), vec![key]);
        }
    }

    #[test]
    fn should_allow_basic_hash_map_operation_on_a_lot_of_keys_across_multiple_pages() {
        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        let bpm = Arc::new(BufferPoolManager::new(4, disk_manager, Some(2), None));

        type Key = u64;
        type Value = u64;

        let mut hash_table = DiskExtendibleHashTable::<
            { hash_table_bucket_array_size::<Key, Value>() },
            Key,
            Value,
            U64Comparator,
            U64IdentityKeyHasher,
        >::new(
            "temp".to_string(),
            bpm,
            U64Comparator,

            // TODO - change to `None`
            Some(2),
            Some(5),
            None,
        );

        // Having enough keys so a split would happen
        let total = (BUSTUB_PAGE_SIZE * 100) as Key;

        // Should not find any values before init
        let tmp = (0..total).shuffle();
        for &key in &tmp[0..10] {
            assert_eq!(hash_table.get_value(&key, None), vec![], "should not find values for key {}", key);
        }

        hash_table.verify_integrity();

        // insert a few (key, value) pairs
        for key in 0..total {


            /// Abort process on panic, this should be used in thread
            // This can lead to corruption, but if we panicked it is a bug in the db (I think)
            assert!(hash_table.insert(&key, &key, None).is_ok(), "should insert new key {}", key);

        }

        hash_table.verify_integrity();

        // Should not find missing keys after the hash map is initialized
        for &key in &(total..total + 1_000_000).shuffle()[0..10] {

            assert_eq!(hash_table.get_value(&key, None), vec![], "should not find values for key {}", key);
        }

        hash_table.verify_integrity();

        // Fetch those in random order
        for key in (0..total).shuffle() {
            assert_eq!(hash_table.get_value(&key, None), vec![key], "should find values for key {}", key);
        }

        hash_table.verify_integrity();

        // Remove 1/7 random keys
        let random_key_index_to_remove = &(0..total).shuffle()[0..(total / 7) as usize];

        for &key in random_key_index_to_remove {
            assert_eq!(hash_table.remove(&key, None).expect("should remove"), true, "should remove key {}", key);
        }

        hash_table.verify_integrity();

        // Fetch all in random order
        {
            let removed_keys = HashSet::<Key>::from_iter(random_key_index_to_remove.iter().cloned());

            for key in (0..total).shuffle() {
                let expected_return = if removed_keys.contains(&key) { vec![] } else { vec![key] };

                assert_eq!(hash_table.get_value(&key, None), expected_return, "get value for key {}", key);
            }
        }

        hash_table.verify_integrity();

        let mut removed_random_keys_to_reinsert = random_key_index_to_remove.iter().cloned().collect::<Vec<_>>().clone();
        removed_random_keys_to_reinsert.shuffle(&mut thread_rng());

        // Add back 1/4 of the removed keys
        let removed_random_keys_to_reinsert = &removed_random_keys_to_reinsert[0..removed_random_keys_to_reinsert.len() / 4];

        let offset_for_reinserted_values = total * 100;

        {
            // insert back some of the removed keys with different values
            for &key in removed_random_keys_to_reinsert {
                let value = key + offset_for_reinserted_values;

    }

    // TODO - add test for deletion to already deleted should not do anything
}
