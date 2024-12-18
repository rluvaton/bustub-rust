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
    use transaction::Transaction;

    type TestKey = u32;
    type TestValue = u64;

    fn panic_on_deadlock() -> (Arc<AtomicBool>, thread::JoinHandle<()>) {
        use std::thread;
        use std::time::Duration;
        use parking_lot::deadlock;

        let finished = Arc::new(AtomicBool::new(false));

        let deadlock_finished = Arc::clone(&finished);

        // Create a background thread which checks for deadlocks every 10s
        let handle = thread::spawn(move || {
            while !deadlock_finished.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(1));
                let deadlocks = deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                println!("{} deadlocks detected", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    println!("Deadlock #{}", i);
                    for t in threads {
                        println!("Thread Id {:?}", t.thread_id());
                        println!("{:#?}", t.backtrace());
                    }
                }
                panic!("Dead lock detected");
            }

            println!("Deadlock detection finished");
        });

        (finished, handle)
    }

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

    fn test_lifecycle<
        const ARRAY_SIZE: usize,
        Key: PageKey,
        Value: PageValue,
        KeyComparator: Comparator<Key>,
        KeyHasherImpl: KeyHasher,
        GetEntryFn: Fn(i64) -> (Key, Value)
    >(hash_table: DiskHashTable<ARRAY_SIZE, Key, Value, KeyComparator, KeyHasherImpl>, total: i64, get_entry_for_index: GetEntryFn) {
        let shuffle_seed: u64 = thread_rng().gen();
        println!("Seed used: {}", shuffle_seed);
        let mut rng = ChaChaRng::seed_from_u64(shuffle_seed);
        let one_percent = total / 100;

        // Should not find any values before init
        println!("Testing trying to find missing values before init");
        let tmp = (0..total).shuffle_with_seed(&mut rng);
        for &i in &tmp[0..10] {
            let (key, _) = get_entry_for_index(i);

            assert_eq!(hash_table.get_value(&key, &Transaction::default()), Ok(vec![]), "should not find values for key {} when seed is {shuffle_seed}", i);
        }

        // Should not delete anything before init
        println!("Testing trying to delete missing values before init");
        let tmp = (0..total).shuffle_with_seed(&mut rng);
        for &i in &tmp[0..10] {
            let (key, _) = get_entry_for_index(i);

            assert_eq!(hash_table.remove(&key, &Transaction::default()), Ok(false), "should not delete for key {} when seed is {shuffle_seed}", i);
        }

        hash_table.with_header_page(|header_page| {
            header_page.verify_empty();
        });

        hash_table.verify_integrity(false);

        println!("Inserting {} entries", total);
        // insert a few (key, value) pairs
        for i in 0..total {
            let (key, value) = get_entry_for_index(i);

            if i % (10 * one_percent) == 0 {
                println!("Inserted {}%", i / one_percent);
            }

            // Abort process on panic, this should be used in thread
            assert_eq!(hash_table.insert(&key, &value, &Transaction::default()), Ok(()), "should insert new key {} when seed is {shuffle_seed}", i);
        }

        println!("All entries inserted");

        hash_table.verify_integrity(false);

        println!("Asserting not finding missing values after init");
        // Should not find missing keys after the hash map is initialized
        for &i in &(total..total + 1_000_000).shuffle_with_seed(&mut rng)[0..10] {
            let (key, _) = get_entry_for_index(i);

            assert_eq!(hash_table.get_value(&key, &Transaction::default()), Ok(vec![]), "should not find values for key {} when seed is {shuffle_seed}", i);
        }

        println!("Asserting not deleting missing values after init");
        // Should not find missing keys after the hash map is initialized
        // Should not find missing keys after the hash map is initialized
        for &i in &(total..total + 1_000_000).shuffle_with_seed(&mut rng)[0..10] {
            let (key, _) = get_entry_for_index(i);

            assert_eq!(hash_table.remove(&key, &Transaction::default()), Ok(false), "should not delete values for key {} when seed is {shuffle_seed}", i);
        }

        hash_table.verify_integrity(false);

        // Fetch those in random order
        println!("Asserting all inserted entries exists");
        {
            let mut counter = 0;
            for i in (0..total).shuffle_with_seed(&mut rng) {
                let (key, value) = get_entry_for_index(i);
                if counter % (10 * one_percent) == 0 {
                    println!("Fetched {}%", counter / one_percent);
                }

                assert_eq!(hash_table.get_value(&key, &Transaction::default()), Ok(vec![value]), "should find values for key {} when seed is {shuffle_seed}", i);

                counter += 1;
            }
        }

        hash_table.verify_integrity(false);

        println!("Remove 1/7 random keys");
        let random_key_index_to_remove = &(0..total).shuffle_with_seed(&mut rng)[0..(total / 7) as usize];

        for &i in random_key_index_to_remove {
            let (key, _) = get_entry_for_index(i);

            assert_eq!(hash_table.remove(&key, &Transaction::default()).expect("should remove"), true, "should remove key {} when seed is {shuffle_seed}", i);
        }

        hash_table.verify_integrity(false);

        println!("Fetch all in random order");
        {
            let removed_keys = HashSet::<i64>::from_iter(random_key_index_to_remove.iter().cloned());

            let mut counter = 0;
            for i in (0..total).shuffle_with_seed(&mut rng) {
                if counter % (10 * one_percent) == 0 {
                    println!("Fetched {}%", counter / one_percent);
                }

                let (key, value) = get_entry_for_index(i);

                let expected_return = if removed_keys.contains(&i) { vec![] } else { vec![value] };

                assert_eq!(hash_table.get_value(&key, &Transaction::default()), Ok(expected_return), "get value for key {} when seed is {shuffle_seed}", i);

                counter += 1;
            }
        }

        hash_table.verify_integrity(false);

        let mut removed_random_keys_to_reinsert = random_key_index_to_remove.iter().cloned().collect::<Vec<_>>().clone();
        removed_random_keys_to_reinsert.shuffle(&mut thread_rng());

        println!("Add back 1/4 of the removed keys with different values");
        // Add back 1/4 of the removed keys
        let removed_random_keys_to_reinsert = &removed_random_keys_to_reinsert[0..removed_random_keys_to_reinsert.len() / 4];

        let offset_for_reinserted_values = total * 100;

        {
            // insert back some of the removed keys with different values
            for &i in removed_random_keys_to_reinsert {
                let (key, _) = get_entry_for_index(i);
                let (_, value) = get_entry_for_index(i + offset_for_reinserted_values);

                assert_eq!(hash_table.insert(&key, &value, &Transaction::default()), Ok(()), "should insert back key {} when seed is {shuffle_seed}", i);
            }
        }

        hash_table.verify_integrity(false);

        println!("Fetch all in random order");

        let reinserted_keys = HashSet::<i64>::from_iter(removed_random_keys_to_reinsert.iter().cloned());
        let removed_keys = HashSet::<i64>::from_iter(random_key_index_to_remove.iter().cloned());

        let removed_keys: HashSet::<i64> = &removed_keys - &reinserted_keys;
        {
            // Fetch all in random order
            let mut counter = 0;
            for i in (0..total).shuffle_with_seed(&mut rng) {
                if counter % (10 * one_percent) == 0 {
                    println!("Fetched {}%", counter / one_percent);
                }

                let (key, value) = get_entry_for_index(i);

                let found_value = hash_table.get_value(&key, &Transaction::default());

                if removed_keys.contains(&i) {
                    assert_eq!(found_value, Ok(vec![]), "should not find any values for removed key {} when seed is {shuffle_seed}", i);
                    continue;
                } else if reinserted_keys.contains(&i) {
                    let (_, value) = get_entry_for_index(i + offset_for_reinserted_values);

                    assert_eq!(found_value, Ok(vec![value]), "should find updated value for reinserted key {} when seed is {shuffle_seed}", i);
                } else {
                    assert_eq!(found_value, Ok(vec![value]), "should find original value for not changed key {} when seed is {shuffle_seed}", i);
                }

                counter += 1;
            }
        }

        hash_table.verify_integrity(false);

        println!("Delete all in random order");
        {
            let mut counter = 0;
            for i in (0..total).shuffle_with_seed(&mut rng) {
                if counter % (10 * one_percent) == 0 {
                    println!("Deleted {}%", counter / one_percent);
                }

                let (key, _) = get_entry_for_index(i);

                let remove_result = hash_table.remove(&key, &Transaction::default());

                if removed_keys.contains(&i) {
                    assert_eq!(remove_result, Ok(false), "should not delete already deleted key, index: {} when seed is {shuffle_seed}", i);
                } else {
                    assert_eq!(remove_result, Ok(true), "should delete key, index: {} when seed is {shuffle_seed}", i);
                }

                if counter % one_percent == 0 {
                    hash_table.verify_integrity(false);
                }

                counter += 1;
            }
        }

        hash_table.verify_integrity(false);
        hash_table.with_header_page(|header_page| {
            header_page.verify_empty();
        });

        println!("Fetch all after everything deleted in random order");
        {
            let mut counter = 0;
            for i in (0..total).shuffle_with_seed(&mut rng) {
                if counter % (10 * one_percent) == 0 {
                    println!("Fetched {}%", counter / one_percent);
                }

                let (key, _) = get_entry_for_index(i);

                assert_eq!(hash_table.get_value(&key, &Transaction::default()), Ok(vec![]), "should not find any values for removed key {} when seed is {shuffle_seed}", i);

                counter += 1;
            }
        }

        hash_table.verify_integrity(false);
    }

    #[test]
    fn should_not_find_anything_before_insertion() {
        let hash_table = create_extendible_hash_table(4);

        // Having enough keys so a split would happen
        let total = PAGE_SIZE * 100;


        for i in 0..total {
            assert_eq!(hash_table.get_value(&(i as TestKey), &Transaction::default()), Ok(vec![]), "should not find values for key {}", i);
        }
    }

    #[test]
    fn lifecycle_small_pool_size() {
        let hash_table = create_extendible_hash_table(4);

        // Having enough keys so a split would happen
        let total = (PAGE_SIZE * 100) as i64;
        test_lifecycle(hash_table, total, |i| (
            i as TestKey,
            i as TestValue,
        ));
    }

    #[test]
    fn lifecycle_large_pool_size() {
        let hash_table = create_extendible_hash_table(1000);

        // Having enough keys so a split would happen
        let total = (PAGE_SIZE * 100) as i64;
        test_lifecycle(hash_table, total, |i| (
            i as TestKey,
            i as TestValue,
        ));
    }

    #[test]
    fn test_from_example_in_single_directory() {
        let bpm = BufferPoolManager::builder()
            .with_pool_size(100)
            .with_disk_manager(DiskManagerUnlimitedMemory::new())
            .with_lru_k_eviction_policy(100)
            .build_arc();

        type Key = u64;
        type Value = u64;

        let hash_table = DiskHashTable::<
            { bucket_array_size::<Key, Value>() },
            Key,
            Value,
            U64Comparator,
            U64IdentityKeyHasher,
        >::new(
            "temp".to_string(),
            bpm,
            OrdComparator::<Key>::default(),

            // 1 directory is enough for us 2^0
            Some(0),

            // Entire entry
            None,

            // can hold up to 3 items
            Some(3),
        ).expect("Should be able to create hash table");

        hash_table.verify_integrity(true);

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
            hash_table.insert(&key, &key, &Transaction::default()).unwrap();
            hash_table.verify_integrity(true);
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
        hash_table.insert(&9, &9, &Transaction::default()).unwrap();
        hash_table.verify_integrity(true);

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
        hash_table.insert(&20, &20, &Transaction::default()).unwrap();
        hash_table.verify_integrity(true);

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
        hash_table.insert(&26, &26, &Transaction::default()).unwrap();
        hash_table.verify_integrity(true);

        let all_keys = [24, 16, 9, 10, 26, 7, 31, 4, 20, 6, 22];

        for key in all_keys {
            assert_eq!(hash_table.get_value(&key, &Transaction::default()), Ok(vec![key]));
        }
    }

    #[test]
    fn lifecycle_small_number_of_keys() {

        let bpm = BufferPoolManager::builder()
            .with_pool_size(4)
            .with_disk_manager(DiskManagerUnlimitedMemory::new())
            .with_lru_k_eviction_policy(2)
            .build_arc();

        type Key = u64;
        type Value = u64;

        let hash_table = DiskHashTable::<
            { bucket_array_size::<Key, Value>() },
            Key,
            Value,
            OrdComparator<Key>,
            U64IdentityKeyHasher,
        >::new(
            "temp".to_string(),
            bpm,
            OrdComparator::<Key>::default(),
            None,
            None,
            None,
        ).expect("Should be able to create hash table");

        // Having enough keys so a split would happen
        let total = PAGE_SIZE as i64;
        test_lifecycle(hash_table, total, |i| (
            i as Key,
            i as Value
        ));
    }

    #[test]
    fn thread_safety_test() {
        let bpm = BufferPoolManager::builder()
            .with_pool_size(100)
            .with_disk_manager(DiskManagerUnlimitedMemory::new())
            .with_lru_k_eviction_policy(2)
            .build_arc();

        type Key = u64;
        type Value = u64;

        let hash_table = Arc::new(DiskHashTable::<
            { bucket_array_size::<Key, Value>() },
            Key,
            Value,
            OrdComparator<Key>,
            DefaultKeyHasher,
        >::new(
            "temp".to_string(),
            bpm,
            OrdComparator::<Key>::default(),
            None,
            None,
            None,
        ).expect("Should be able to create hash table"));

        let total = (PAGE_SIZE * 10) as i64; // Reduce the number of operations for concurrency testing

        // Number of threads to spawn
        let num_threads = 8;
        // Barrier to synchronize thread start
        let barrier = Arc::new(Barrier::new(num_threads));

        let (finished, deadlock_thread_handle) = panic_on_deadlock();

        let mut handles = vec![];

        for thread_id in 0..num_threads {
            let hash_table = Arc::clone(&hash_table);
            let barrier = Arc::clone(&barrier);

            let handle = thread::spawn(move || {
                // Ensure all threads start simultaneously
                barrier.wait();
                let one_percent = total / 100;

                let offset = thread_id as i64 * total;
                for i in 0..total {
                    let key = (i + offset) as Key;
                    let rid = (i + offset) as Value;

                    if i % (10 * one_percent) == 0 {
                        println!("Passed {}%", (i / one_percent))
                    }
                    // Alternating between insert and delete for stress
                    if i % 2 == 0 {
                        let result = hash_table.insert(&key, &rid, &Transaction::default());

                        assert_eq!(
                            result,
                            Ok(()),
                            "Thread {} failed to insert key {}",
                            thread_id,
                            i + offset
                        );
                    } else {
                        let result = hash_table.get_value(&key, &Transaction::default());

                        assert!(result.is_ok(), "Thread {} failed to get key {}", thread_id, i + offset);
                    }

                    if i % 3 == 0 {
                        let result = hash_table.remove(&key, &Transaction::default());

                        match result {
                            Ok(_) => {}
                            Err(err) => {
                                // Force failure
                                assert_eq!(Some(err), None, "Thread {} failed to remove key {}", thread_id, i + offset);
                            }
                        }
                    }
                }

                println!("Thread {} Finished", thread_id);

                hash_table.verify_integrity(false);
            });

            handles.push(handle);
        }

        // Wait for all threads to finish
        for handle in handles {
            handle.join().expect("Thread failed");
        }

        finished.store(true, Ordering::Relaxed);
        deadlock_thread_handle.join().unwrap();

        println!("All threads finished");

        // Final integrity check
        hash_table.verify_integrity(false);
    }

    // TODO - add test for deletion to already deleted should not do anything
}
