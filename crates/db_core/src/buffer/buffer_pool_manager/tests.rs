#[cfg(test)]
mod tests {
    use crate::buffer::buffer_pool_manager::errors::{FetchPageError, NewPageError, NoAvailableFrameFound};
    use crate::buffer::{AccessType};
    use crate::buffer::buffer_pool_manager::{BufferPool, BufferPoolManager, PageWriteGuard};
    use crate::storage::{AlignToPageData, DefaultDiskManager};
    use common::config::{PageId, BUSTUB_PAGE_SIZE};
    use parking_lot::{Condvar, Mutex};
    use rand::Rng;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    use tempdir::TempDir;

    fn setup() -> TempDir {
        TempDir::new("buffer_pool_manager_tests").expect("Should create tmp directory")
    }

    // Check whether pages containing terminal characters can be recovered
    #[test]
    fn binary_data() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let mut rng = rand::thread_rng();

        let lower_bound = u8::MIN;
        let upper_bound = u8::MAX;

        // No matter if `char` is signed or unsigned by default, this constraint must be met
        assert_eq!(upper_bound - lower_bound, 255);

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        // Scenario: The buffer pool is empty. We should be able to create a new page.
        let mut page0 = bpm.new_page(AccessType::Unknown).expect("The buffer pool is empty. We should be able to create a new page");

        assert_eq!(page0.get_page_id(), 0);

        // Generate random binary data
        let mut random_binary_data: [u8; BUSTUB_PAGE_SIZE] = [0; BUSTUB_PAGE_SIZE];

        random_binary_data.fill_with(|| rng.gen_range(lower_bound..upper_bound) as u8);

        // Insert terminal characters both in the middle and at end
        random_binary_data[BUSTUB_PAGE_SIZE / 2] = 0;
        random_binary_data[BUSTUB_PAGE_SIZE - 1] = 0;

        // Scenario: Once we have a page, we should be able to read and write content.
        page0.get_data_mut().copy_from_slice(&random_binary_data);
        assert_eq!(page0.get_data(), random_binary_data.as_slice());

        let mut page_guards = vec![];

        page_guards.push(page0);

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for _ in 1..buffer_pool_size {
            page_guards.push(bpm.new_page(AccessType::Unknown).expect("Should be able to create new page"));
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for _ in buffer_pool_size..buffer_pool_size * 2 {
            assert_eq!(bpm.new_page(AccessType::Unknown).expect_err("Buffer pool is full"), NoAvailableFrameFound.into());
        }

        // Scenario: After unpinning pages {0, 1, 2, 3, 4}, we should be able to create 5 new pages
        for _ in 0..5 {
            let page_id: PageId = page_guards.remove(0).get_page_id();

            assert_eq!(bpm.flush_page(page_id), true, "should be able to flush page {}", page_id)
        }

        for _ in 0..5 {
            let _ = bpm.new_page(AccessType::Unknown).expect("Must be able to create a new page");
        }

        // Scenario: We should be able to fetch the data we wrote a while ago.
        let page0 = bpm.fetch_page_read(0, AccessType::default())
            .expect("We should be able to fetch the data we wrote a while ago");

        assert_eq!(page0.get_data(), random_binary_data.as_slice());

        assert_eq!(bpm.unpin_page(0, AccessType::default()), true);

        // Shutdown the disk manager and remove the temporary file we created.
        // TODO - shutdown
        // disk_manager.shut_down();
        // remove("test.db");
        //
        // delete bpm;
        // delete disk_manager;
    }

    #[test]
    fn sample() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        // Scenario: The buffer pool is empty. We should be able to create a new page.
        let mut page0 = bpm.new_page(AccessType::Unknown).expect("The buffer pool is empty. We should be able to create a new page");

        assert_eq!(page0.get_page_id(), 0);

        // Scenario: Once we have a page, we should be able to read and write content.

        let expected_data = "Hello";

        page0.get_data_mut()[..expected_data.len()].copy_from_slice(expected_data.as_bytes());
        // this is different from the original test that check the entire data
        assert_eq!(page0.get_data(), &expected_data.as_bytes().align_to_page_data());

        let mut page_guards = vec![];

        page_guards.push(page0);

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for _ in 1..buffer_pool_size {
            page_guards.push(bpm.new_page(AccessType::Unknown).expect("Should be able to create new page"));
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for _ in buffer_pool_size..buffer_pool_size * 2 {
            assert_eq!(bpm.new_page(AccessType::Unknown).expect_err("buffer pool is full"), NoAvailableFrameFound.into());
        }

        // Scenario: After unpinning pages {0, 1, 2, 3, 4} and pinning another 4 new pages,
        // there would still be one buffer page left for reading page 0.
        for i in 0..5 {
            // Should drop guard and unpin
            let _ = page_guards.remove(0);
        }

        for _ in 0..4 {
            page_guards.push(bpm.new_page(AccessType::Unknown).expect("Should be able to create new page"));
        }

        // Scenario: We should be able to fetch the data we wrote a while ago.
        let page0 = bpm.fetch_page_read(0, AccessType::default()).expect("should be able to fetch the data we wrote a while ago");
        assert_eq!(page0.get_data(), &"Hello".as_bytes().align_to_page_data());

        // Scenario: If we unpin page 0 and then make a new page, all the buffer pages should
        // now be pinned. Fetching page 0 again should fail.
        drop(page0);
        page_guards.push(bpm.new_page(AccessType::Unknown).expect("Should be able to create new page"));
        assert_eq!(bpm.fetch_page_read(0, AccessType::default()).expect_err("buffer pool is full"), NoAvailableFrameFound.into());

        // Shutdown the disk manager and remove the temporary file we created.
        // TODO - shutdown
        // disk_manager.shut_down();
        // remove("test.db");
        //
        // delete bpm;
        // delete disk_manager;
    }

    #[test]
    fn basic_thread_safe() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        let created_page_id = bpm.new_page(AccessType::Unknown).expect("should create new page").get_page_id();

        let wait_for_next = Arc::new(AtomicUsize::new(0));

        let wait_for_next_thread_1 = wait_for_next.clone();
        let wait_for_next_thread_2 = wait_for_next.clone();
        let bpm_thread_1 = bpm.clone();

        // Thread 1
        let thread_1 = thread::spawn(move || {
            // Fetch the created page with write lock
            let fetch_page_with_write = bpm_thread_1.fetch_page_write(created_page_id, AccessType::Unknown).expect("Should be able to fetch page");

            // Page fetched, Release lock so the next thread can now fetch
            wait_for_next_thread_1.store(1, Ordering::SeqCst);

            // Wait for the next thread to lock
            while wait_for_next_thread_1.load(Ordering::SeqCst) == 1 {
                thread::sleep(Duration::from_millis(1));
            }

            // Wait for the next thread to hold the buffer pool manager root lock
            thread::sleep(Duration::from_millis(50));

            println!("{}", wait_for_next_thread_1.load(Ordering::SeqCst));


            // Try to create new page while the other thread fetching page
            let _ = bpm_thread_1.new_page(AccessType::Unknown).expect("Should be able to create page");

            println!("{}", wait_for_next_thread_1.load(Ordering::SeqCst));


            // Release both pages
        });


        let bpm_thread_2 = bpm.clone();
        let thread_2 = thread::spawn(move || {
            // Wait for the buffer to fetch the page in buffer
            while wait_for_next_thread_2.load(Ordering::SeqCst) != 1 {
                thread::sleep(Duration::from_millis(1));
            }

            wait_for_next_thread_2.store(2, Ordering::SeqCst);

            // Fetch the same page as write
            let _ = bpm_thread_2.fetch_page_write(created_page_id, AccessType::Unknown).expect("Should be able to fetch page");

            wait_for_next_thread_2.store(3, Ordering::SeqCst);
        });

        thread_1.join().unwrap();
        thread_2.join().unwrap();

        println!("{}", wait_for_next.load(Ordering::SeqCst))
    }

    #[test]
    fn page_pin_easy() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(2, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        let page_id_0: PageId;
        let page_id_1: PageId;

        {
            page_id_0 = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();
            let mut page0_write = bpm.fetch_page_write(page_id_0, AccessType::Unknown).expect("Should be able to fetch page");
            {
                let data = "page0";
                page0_write.get_data_mut()[..data.len()].copy_from_slice(data.as_bytes());
            }

            page_id_1 = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();
            let mut page1_write = bpm.fetch_page_write(page_id_1, AccessType::Unknown).expect("Should be able to fetch page");
            {
                let data = "page1";
                page1_write.get_data_mut()[..data.len()].copy_from_slice(data.as_bytes());
            }

            assert_eq!(bpm.get_pin_count(page_id_0), Some(1));
            assert_eq!(bpm.get_pin_count(page_id_1), Some(1));

            let temp_page1 = bpm.new_page(AccessType::Unknown).expect_err("should not be able to create new page when buffer is full");
            assert_eq!(temp_page1, NewPageError::NoAvailableFrameFound);

            let temp_page2 = bpm.new_page(AccessType::Unknown).expect_err("should not be able to create new page when buffer is full");
            assert_eq!(temp_page2, NewPageError::NoAvailableFrameFound);


            assert_eq!(bpm.get_pin_count(page_id_0), Some(1));
            drop(page0_write);
            assert_eq!(bpm.get_pin_count(page_id_0), Some(0));

            assert_eq!(bpm.get_pin_count(page_id_1), Some(1));
            drop(page1_write);
            assert_eq!(bpm.get_pin_count(page_id_1), Some(0));
        }

        {
            let temp_page_1 = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");
            let temp_page_2 = bpm.new_page(AccessType::Unknown).expect("Should be able to create new page");

            assert_eq!(bpm.get_pin_count(page_id_0), None);
            assert_eq!(bpm.get_pin_count(page_id_1), None);
        }

        {
            let mut page0_write = bpm.fetch_page_write(page_id_0, AccessType::Unknown).expect("Should be able to fetch page");
            assert_eq!(page0_write.get_data().as_slice(), "page0".align_to_page_data().as_slice());
            {
                let data = "page0updated";
                page0_write.get_data_mut()[..data.len()].copy_from_slice(data.as_bytes());
            }

            let mut page1_write = bpm.fetch_page_write(page_id_1, AccessType::Unknown).expect("Should be able to fetch page");
            assert_eq!(page1_write.get_data().as_slice(), "page1".align_to_page_data().as_slice());
            {
                let data = "page1updated";
                page1_write.get_data_mut()[..data.len()].copy_from_slice(data.as_bytes());
            }

            assert_eq!(bpm.get_pin_count(page_id_0), Some(1));
            assert_eq!(bpm.get_pin_count(page_id_1), Some(1));
        }

        assert_eq!(bpm.get_pin_count(page_id_0), Some(0));
        assert_eq!(bpm.get_pin_count(page_id_1), Some(0));

        {
            let page0_read = bpm.fetch_page_read(page_id_0, AccessType::Unknown).expect("Should be able to fetch page");
            assert_eq!(page0_read.get_data().as_slice(), "page0updated".align_to_page_data().as_slice());

            let page1_read = bpm.fetch_page_read(page_id_1, AccessType::Unknown).expect("Should be able to fetch page");
            assert_eq!(page1_read.get_data().as_slice(), "page1updated".align_to_page_data().as_slice());

            assert_eq!(bpm.get_pin_count(page_id_0), Some(1));
            assert_eq!(bpm.get_pin_count(page_id_1), Some(1));
        }

        assert_eq!(bpm.get_pin_count(page_id_0), Some(0));
        assert_eq!(bpm.get_pin_count(page_id_1), Some(0));
    }

    #[test]
    fn page_pin_medium() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None);


        // Scenario: The buffer pool is empty. We should be able to create a new page.
        let mut page0 = bpm.new_page(AccessType::Unknown).unwrap();
        let pid0 = page0.get_page_id();


        // Scenario: Once we have a page, we should be able to read and write content.
        page0.get_data_mut().copy_from_slice("Hello".align_to_page_data().as_slice());
        assert_eq!(page0.get_data_mut().as_slice(), "Hello".align_to_page_data().as_slice());

        drop(page0);

        // Create a vector of unique pointers to page guards, which prevents the guards from getting destructed.
        let mut pages: Vec<PageWriteGuard> = vec![];

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for _ in 0..buffer_pool_size {
            pages.push(bpm.new_page(AccessType::Unknown).unwrap());
        }

        // Scenario: All the pin counts should be 1.
        for page in &pages {
            let pid = page.get_page_id();

            assert_eq!(bpm.get_pin_count(pid), Some(1));
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for _ in 0..buffer_pool_size {
            let attempt_new_page_error = bpm.new_page(AccessType::Unknown).expect_err("Should fail to create new page as the buffer pool is full");

            assert_eq!(attempt_new_page_error, NewPageError::NoAvailableFrameFound);
        }

        // Scenario: Drop the first 5 pages to unpin them.
        for _ in 0..(buffer_pool_size / 2) {
            let pid = pages[0].get_page_id();

            assert_eq!(bpm.get_pin_count(pid), Some(1));
            {
                let _ = pages.remove(0);
            }

            assert_eq!(bpm.get_pin_count(pid), Some(0));
        }

        // Scenario: All the pin counts of the pages we haven't dropped yet should still be 1.
        for page in &pages {
            let pid = page.get_page_id();

            assert_eq!(bpm.get_pin_count(pid), Some(1));
        }


        // Scenario: After unpinning pages {1, 2, 3, 4, 5}, we should be able to create 4 new pages and bring them into
        // memory. Bringing those 4 pages into memory should evict the first 4 pages {1, 2, 3, 4} because of LRU.
        for i in 0..((buffer_pool_size / 2) - 1) {
            pages.push(bpm.new_page(AccessType::Unknown).unwrap());
        }

        // Scenario: There should be one frame available, and we should be able to fetch the data we wrote a while ago.
        {
            let original_page = bpm.fetch_page_read(pid0, AccessType::Unknown).expect("Should be able to fetch");

            assert_eq!(original_page.get_data().as_slice(), "Hello".align_to_page_data().as_slice());
        }


        // Scenario: Once we unpin page 0 and then make a new page, all the buffer pages should now be pinned. Fetching page 0
        // again should fail.
        let last_pid = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();
        let last_page = bpm.fetch_page_read(last_pid, AccessType::Unknown);

        let fail = bpm.fetch_page_read(pid0, AccessType::Unknown).expect_err("Should fail to fetch page when buffer pool is full");
        assert_eq!(fail, FetchPageError::NoAvailableFrameFound);
    }

    #[test]
    fn page_access_test() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(1, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        const ROUNDS: usize = 50;

        let pid = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();

        let thread = {
            let bpm = Arc::clone(&bpm);
            thread::spawn(move || {
                // The writer can keep writing to the same page.
                for i in 0..ROUNDS {
                    thread::sleep(Duration::from_millis(5));

                    let mut guard = bpm.fetch_page_write(pid, AccessType::Unknown).expect("Should be able to fetch page");

                    let data = i.to_string();
                    guard.get_data_mut()[0..data.len()].copy_from_slice(data.as_bytes());
                }
            })
        };

        for i in 0..ROUNDS {
            // Wait for a bit before taking the latch, allowing the writer to write some stuff.
            thread::sleep(Duration::from_millis(10));

            // While we are reading, nobody should be able to modify the data.
            let guard = bpm.fetch_page_read(pid, AccessType::Unknown).expect("Should be able to fetch page as read");

            // Save the data we observe.
            let buf = guard.get_data().as_slice();

            // Sleep for a bit. If latching is working properly, nothing should be writing to the page.
            thread::sleep(Duration::from_millis(10));

            // Check that the data is unmodified.
            assert_eq!(guard.get_data().as_slice(), buf);
        }

        thread.join().unwrap();
    }

    #[test]
    fn contention_test() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        const ROUNDS: usize = 100000;

        let pid = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();

        let mut threads = vec![];

        for _ in 0..4 {
            let bpm = Arc::clone(&bpm);
            let thread = thread::spawn(move || {
                for i in 0..ROUNDS {
                    let mut guard = bpm.fetch_page_write(pid, AccessType::Unknown).expect("Should be able to fetch page");

                    let data = i.to_string();
                    guard.get_data_mut()[0..data.len()].copy_from_slice(data.as_bytes());
                }
            });

            threads.push(thread);
        }

        let thread4 = threads.pop().unwrap();
        let thread3 = threads.pop().unwrap();
        let thread2 = threads.pop().unwrap();
        let thread1 = threads.pop().unwrap();

        thread3.join().unwrap();
        thread2.join().unwrap();
        thread4.join().unwrap();
        thread1.join().unwrap();
    }

    #[test]
    fn deadlock_test() {
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let buffer_pool_size = 10;
        let k = 5;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        let pid0 = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();
        let pid1 = bpm.new_page(AccessType::Unknown).unwrap().get_page_id();

        let guard0 = bpm.fetch_page_write(pid0, AccessType::Unknown);

        // A crude way of synchronizing threads, but works for this small case.
        let start = Arc::new(AtomicBool::new(false));

        let bpm_child_thread = Arc::clone(&bpm);
        let start_child_thread = Arc::clone(&start);
        let child_thread = thread::spawn(move || {
            // Acknowledge that we can begin the test.
            start_child_thread.store(true, Ordering::SeqCst);

            // Attempt to write to page 0.
            let guard0 = bpm_child_thread.fetch_page_write(pid0, AccessType::Unknown);
        });

        // Wait for the other thread to begin before we start the test.
        while !start.load(Ordering::SeqCst) {}

        // Make the other thread wait for a bit.
        // This mimics the main thread doing some work while holding the write latch on page 0.
        thread::sleep(Duration::from_secs(1));


        // If your latching mechanism is incorrect, the next line of code will deadlock.
        // Think about what might happen if you hold a certain "all-encompassing" latch for too long...

        // While holding page 0, take the latch on page 1.
        let guard1 = bpm.fetch_page_write(pid1, AccessType::Unknown);

        // Let the child thread have the page 0 since we're done with it.
        drop(guard0);

        child_thread.join().unwrap();
    }

    #[ignore]
    #[test]
    fn evictable_test() {
        // Test if the evictable status of a frame is always correct.
        let tmpdir = setup();
        let db_name = tmpdir.path().join("test.db");
        let k = 5;

        let rounds = 1000;
        let num_readers = 8;

        let disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");

        // Only allocate 1 frame of memory to the buffer pool manager.
        let bpm = BufferPoolManager::new(1, Arc::new(Mutex::new(disk_manager)), Some(k), None);

        for i in 0..rounds {
            // This signal tells the readers that they can start reading after the main thread has already taken the read latch.
            let signal = Arc::new((Mutex::new(false), Condvar::new()));

            // This page will be loaded into the only available frame.
            let winner_pid: PageId = bpm.new_page(AccessType::Unknown).expect("should be able to create new page").get_page_id();

            // We will attempt to load this page into the occupied frame, and it should fail every time.
            // the first creation it should work
            let loser_pid: PageId = bpm.new_page(AccessType::Unknown).expect("should be able to create new page").get_page_id();

            let mut readers = vec![];

            for j in 0..num_readers {
                let bpm = Arc::clone(&bpm);
                let signal = Arc::clone(&signal);
                let reader = thread::spawn(move || {
                    let &(ref lock, ref cvar) = &*signal;
                    let mut started = lock.lock();

                    // Wait until the main thread has taken a read latch on the page.
                    while !*started {
                        cvar.wait(&mut started);
                    }

                    // Read the page in shared mode.
                    let read_guard = bpm.fetch_page_read(winner_pid, AccessType::Unknown).expect("Should be able to read the winner page id");

                    // Since the only frame is pinned, no thread should be able to bring in a new page.
                    let attempt_read_page_error = bpm.fetch_page_read(loser_pid, AccessType::Unknown).expect_err("Should fail to bring another page in");
                    assert_eq!(attempt_read_page_error, FetchPageError::NoAvailableFrameFound);
                });

                readers.push(reader);
            }

            let &(ref lock, ref cvar) = &*signal;
            let mut started = lock.lock();

            if i % 2 == 0 {
                // Take the read latch on the page and pin it.
                let read_guard = bpm.fetch_page_read(winner_pid, AccessType::Unknown);

                // Wake up all the readers.
                *started = true;
                cvar.notify_one();
                drop(started);

                // Allow other threads to read.
                drop(read_guard);
            } else {
                // Take the read latch on the page and pin it.
                let write_guard = bpm.fetch_page_write(winner_pid, AccessType::Unknown);

                // Wake up all the readers.
                *started = true;
                cvar.notify_one();
                drop(started);

                // Allow other threads to read.
                drop(write_guard);
            }

            for reader in readers {
                reader.join().unwrap()
            }
        }
    }
}

