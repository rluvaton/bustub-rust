#[cfg(test)]
mod tests {
    use buffer_common::{AccessType, FrameId};

    use rand::Rng;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::Arc;
    use std::thread;
    use std::thread::{sleep, JoinHandle};
    use std::time::Duration;
    use parking_lot::Mutex;
    use crate::{LRUKEvictionPolicy, EvictionPolicy};
    use crate::lru_k::LRUKOptions;
    use crate::traits::EvictionPolicyCreator;

    #[test]
    fn sample() {
        let mut lru_replacer: LRUKEvictionPolicy = LRUKEvictionPolicy::new(7, LRUKOptions::new(2));

        // Scenario: add six elements to the replacer. We have [1,2,3,4,5]. Frame 6 is non-evictable.
        lru_replacer.record_access(1, AccessType::default());
        lru_replacer.record_access(2, AccessType::default());
        lru_replacer.record_access(3, AccessType::default());
        lru_replacer.record_access(4, AccessType::default());
        lru_replacer.record_access(5, AccessType::default());
        lru_replacer.record_access(6, AccessType::default());
        lru_replacer.set_evictable(1, true);
        lru_replacer.set_evictable(2, true);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        lru_replacer.set_evictable(5, true);
        lru_replacer.set_evictable(6, false);
        assert_eq!(lru_replacer.size(), 5);

        assert_eq!(lru_replacer.get_order_of_eviction(), vec![1, 2, 3, 4, 5]);

        // Scenario: Insert access history for frame 1. Now frame 1 has two access histories.
        // All other frames have max backward k-dist. The order of eviction is [2,3,4,5,1].
        lru_replacer.record_access(1, AccessType::default());
        assert_eq!(lru_replacer.get_order_of_eviction(), vec![2, 3, 4, 5, 1]);

        // Scenario: Evict three pages from the replacer. Elements with max k-distance should be popped
        // first based on LRU.
        assert_eq!(lru_replacer.evict(), Some(2));
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.evict(), Some(4));
        assert_eq!(lru_replacer.size(), 2);

        // Scenario: Now replacer has frames [5,1].
        assert_eq!(lru_replacer.get_order_of_eviction(), vec![5, 1]);

        // Insert new frames 3, 4, and update access history for 5. We should end with [3,1,5,4]
        lru_replacer.record_access(3, AccessType::default());
        lru_replacer.record_access(4, AccessType::default());
        lru_replacer.record_access(5, AccessType::default());
        lru_replacer.record_access(4, AccessType::default());
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        assert_eq!(lru_replacer.size(), 4);
        assert_eq!(lru_replacer.get_order_of_eviction(), vec![3, 1, 5, 4]);


        // Scenario: continue looking for victims. We expect 3 to be evicted next.
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.size(), 3);

        // Set 6 to be evictable. 6 Should be evicted next since it has max backward k-dist.
        lru_replacer.set_evictable(6, true);
        assert_eq!(lru_replacer.size(), 4);
        assert_eq!(lru_replacer.evict(), Some(6));
        assert_eq!(lru_replacer.size(), 3);

        // Now we have [1,5,4]. Continue looking for victims.
        assert_eq!(lru_replacer.get_order_of_eviction(), vec![1, 5, 4]);

        lru_replacer.set_evictable(1, false);
        assert_eq!(lru_replacer.size(), 2);
        assert_eq!(lru_replacer.evict(), Some(5));
        assert_eq!(lru_replacer.size(), 1);

        // Update access history for 1. Now we have [4,1]. Next victim is 4.
        lru_replacer.record_access(1, AccessType::default());
        lru_replacer.record_access(1, AccessType::default());
        lru_replacer.set_evictable(1, true);
        assert_eq!(lru_replacer.get_order_of_eviction(), vec![4, 1]);
        assert_eq!(lru_replacer.size(), 2);
        assert_eq!(lru_replacer.evict(), Some(4));

        assert_eq!(lru_replacer.size(), 1);
        assert_eq!(lru_replacer.evict(), Some(1));
        assert_eq!(lru_replacer.size(), 0);

        // This operation should not modify size
        assert_eq!(lru_replacer.evict(), None);
        assert_eq!(lru_replacer.size(), 0);
    }

    #[test]
    fn two_thread_recording_access() {
        // This does not check any correctness, it just checks that we can use the LRU with threads

        const FRAMES: usize = 50;
        let lru_replacer = Arc::new(Mutex::new(LRUKEvictionPolicy::new(FRAMES, LRUKOptions::new(10))));

        let mut threads: Vec<JoinHandle<()>> = vec![];

        // run 2 threads
        for _ in 0..2 {
            let mut lru_replacer = lru_replacer.clone();

            let t = thread::spawn(move || {

                // In each one record access to every frame
                for i in 0..FRAMES {
                    lru_replacer.lock().record_access(i as FrameId, AccessType::default());
                    lru_replacer.lock().set_evictable(i as FrameId, true);
                }
            });

            threads.push(t);
        }

        // run recording access and evictable threads
        for t in threads {
            t.join().expect("join must work");
        }

        assert_eq!(lru_replacer.lock().size(), FRAMES);
    }

    #[test]
    fn many_threads() {
        // This does not check any correctness, it just check that we can use the lru with threads
        const FRAMES: usize = 1000;
        let lru_replacer = Arc::new(Mutex::new(LRUKEvictionPolicy::new(FRAMES, LRUKOptions::new(10))));

        let mut threads: Vec<JoinHandle<()>> = vec![];

        for _ in 0..10 {
            let mut lru_replacer = lru_replacer.clone();

            let t = thread::spawn(move || {

                for _ in 0..FRAMES * 10 {
                    let frame_id: FrameId = rand::thread_rng().gen_range(1..FRAMES) as FrameId;

                    lru_replacer.lock().record_access(frame_id, AccessType::default());
                }
            });

            threads.push(t);
        }

        for _ in 0..4 {
            let mut lru_replacer = lru_replacer.clone();

            let t = thread::spawn(move || {
                for _ in 0..FRAMES * 10 {
                    let frame_id: FrameId = rand::thread_rng().gen_range(1..FRAMES) as FrameId;
                    let evictable = rand::thread_rng().gen_range(1..=2) == 1;
                    lru_replacer.lock().set_evictable(frame_id, evictable);
                }
            });

            threads.push(t);
        }

        let mut eviction_threads: Vec<JoinHandle<()>> = vec![];
        let stop = Arc::new(AtomicBool::new(false));

        for _ in 0..4 {
            let stop = Arc::clone(&stop);
            let mut lru_replacer = lru_replacer.clone();
            let t = thread::spawn(move || {

                while !stop.load(SeqCst) {
                    lru_replacer.lock().evict();

                    let sleep_microseconds = rand::thread_rng().gen_range(1..1000);

                    sleep(Duration::from_micros(sleep_microseconds));
                }
            });

            eviction_threads.push(t);
        }

        // run recording access and evictable threads
        for t in threads {
            t.join().expect("join must work");
        }

        stop.store(true, SeqCst);

        for t in eviction_threads {
            t.join().expect("join must work");
        }
    }

    #[test]
    fn concurrent_set_evictable() {
        // This does not check any correctness, it just check that we can use the lru with threads
        const FRAMES: usize = 10;
        let mut lru_replacer = Arc::new(Mutex::new(LRUKEvictionPolicy::new(FRAMES, LRUKOptions::new(10))));

        for i in 0..FRAMES {
            lru_replacer.lock().record_access(i as FrameId, AccessType::default());
        }

        let mut threads: Vec<JoinHandle<()>> = vec![];

        for _ in 0..1000 {
            let mut lru_replacer = lru_replacer.clone();
            let t = thread::spawn(move || {

                for i in 0..FRAMES {
                    lru_replacer.lock().set_evictable(i as FrameId, false);
                    lru_replacer.lock().set_evictable(i as FrameId, true);
                }

            });

            threads.push(t);
        }

        // run recording access and evictable threads
        for t in threads {
            t.join().expect("join must work");
        }

        assert_eq!(lru_replacer.lock().size(), FRAMES);
    }

    #[test]
    fn concurrent_evict_and_access() {
        let mut lru_replacer = Arc::new(Mutex::new(LRUKEvictionPolicy::new(7, LRUKOptions::new(2))));
        lru_replacer.lock().record_access(1, AccessType::default());
        lru_replacer.lock().set_evictable(1, true);

        let mut evict_replacer = lru_replacer.clone();
        let evict_handle = thread::spawn(move || {
            evict_replacer.lock().evict();
        });

        let mut access_replacer = lru_replacer.clone();
        let access_handle = thread::spawn(move || {
            access_replacer.lock().record_access(1, AccessType::default());
        });

        evict_handle.join().unwrap();
        access_handle.join().unwrap();

        assert!(lru_replacer.lock().size() <= 1); // Either evicted or recorded
    }
}
