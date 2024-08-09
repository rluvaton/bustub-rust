#[cfg(test)]
mod tests {
    use crate::trie_store::TrieStore;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::Arc;
    use std::thread;
    use std::thread::JoinHandle;

    #[test]
    fn basic_put() {
        let mut store = TrieStore::new();

        assert_eq!(store.get("233"), None);

        store.put("233", 2333.into());

        {
            assert_eq!(store.get("233"), Some(2333.into()));
        }

        store.remove("233");

        {
            assert_eq!(store.get("233"), None);
        }
    }

    #[test]
    fn guard() {
        let mut store = TrieStore::new();

        assert_eq!(store.get("233"), None);

        store.put("233", "2333".to_string().into());

        let guard = store.get("233");
        assert_eq!(guard, Some("2333".to_string().into()));

        store.remove("233");

        {
            assert_eq!(store.get("233"), None);
        }

        assert_eq!(guard, Some("2333".to_string().into()));
    }

    #[test]
    fn mixed() {
        // TODO - this is very slow, probably because all of the cloning
        let mut store = TrieStore::new();

        for i in 0..23333 {
            let key = format!("{:#05}", i);
            let value = format!("value-{:#08}", i);

            // I need each put to transfer ownership of the entire trie
            store.put(key.as_str(), value.into());
        }

        for i in (0..23333).step_by(2) {
            let key = format!("{:#05}", i);
            let value = format!("new-value-{:#08}", i);

            store.put(key.as_str(), value.into());
        }

        for i in (0..23333).step_by(3) {
            let key = format!("{:#05}", i);

            store.remove(key.as_str());
        }

        // verify final trie
        for i in 0..23333 {
            let key = format!("{:#05}", i);
            if i % 3 == 0 {
                assert_eq!(store.get(key.as_str()), None);
            } else if i % 2 == 0 {
                let value = format!("new-value-{:#08}", i);
                assert_eq!(store.get(key.as_str()), Some(value.into()));
            } else {
                let value = format!("value-{:#08}", i);
                assert_eq!(store.get(key.as_str()), Some(value.into()));
            }
        }
    }

    #[test]
    fn mixed_concurrent() {
        // TODO - this is very slow, probably because all of the cloning
        let store = Arc::new(TrieStore::new());

        let mut write_threads: Vec<JoinHandle<()>> = vec![];

        const KEYS_PER_THREAD: u32 = 10_000;

        for tid in 0..4 {
            let mut store = Arc::clone(&store);

            let t = thread::spawn(move || {
                let store = Arc::make_mut(&mut store);

                for i in 0..KEYS_PER_THREAD {
                    let key = format!("{:#05}", i * 4 + tid);
                    let value = format!("value-{:#08}", i * 4 + tid);

                    store.put(key.as_str(), value.into());
                }

                for i in 0..KEYS_PER_THREAD {
                    let key = format!("{:#05}", i * 4 + tid);

                    store.remove(key.as_str());
                }

                for i in 0..KEYS_PER_THREAD {
                    let key = format!("{:#05}", i * 4 + tid);
                    let value = format!("new-value-{:#08}", i * 4 + tid);

                    store.put(key.as_str(), value.into());
                }
            });

            write_threads.push(t);
        }

        let mut read_threads: Vec<JoinHandle<()>> = vec![];
        let stop = Arc::new(AtomicBool::new(false));

        for tid in 0..4 {
            let store = Arc::clone(&store);
            let stop = Arc::clone(&stop);
            let t = thread::spawn(move || {
                let mut i = 0u32;

                while !stop.load(SeqCst) {
                    let key = format!("{:#05}", i * 4 + tid);

                    store.get(key.as_str());

                    i = (i + 1) % KEYS_PER_THREAD;
                }
            });

            read_threads.push(t);
        }

        // run write threads
        for t in write_threads {
            t.join().expect("join must work");
        }

        stop.store(true, SeqCst);

        for t in read_threads {
            t.join().expect("join must work");
        }

        // verify final trie
        for i in 0..KEYS_PER_THREAD * 4 {
            let key = format!("{:#05}", i);
            let value = format!("new-value-{:#08}", i);
            assert_eq!(store.get(key.as_str()), Some(value.into()));
        }
    }

    #[test]
    fn non_copyable() {
        let mut store = TrieStore::new();

        store.put("tes", 233.into());
        store.put("te", 23.into());
        store.put("test", 2333.into());

        assert_eq!(store.get("te"), Some(23.into()));
        assert_eq!(store.get("tes"), Some(233.into()));
        assert_eq!(store.get("test"), Some(2333.into()));

        store.remove("te");
        store.remove("tes");
        store.remove("test");

        assert_eq!(store.get("te"), None);
        assert_eq!(store.get("tes"), None);
        assert_eq!(store.get("test"), None);
    }

    // Did not implement the ReadWriteTest as I don't understand it
}
