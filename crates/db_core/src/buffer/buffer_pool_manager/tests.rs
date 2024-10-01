#[cfg(test)]
mod tests {
    use crate::buffer::{AccessType, BufferPoolManager};
    use crate::storage::{AlignToPageData, DefaultDiskManager};
    use common::config::BUSTUB_PAGE_SIZE;
    use parking_lot::Mutex;
    use rand::Rng;
    use std::sync::Arc;
    use std::{mem, thread};
    use std::ops::{Deref, DerefMut};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;
    use crate::buffer::errors::NoAvailableFrameFound;
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
        let page0 = bpm.new_page().expect("The buffer pool is empty. We should be able to create a new page");

        assert_eq!(page0.with_read(|u| u.get_page_id()), 0);

        // Generate random binary data
        let mut random_binary_data: [u8; BUSTUB_PAGE_SIZE] = [0; BUSTUB_PAGE_SIZE];

        random_binary_data.fill_with(|| rng.gen_range(lower_bound..upper_bound) as u8);

        // Insert terminal characters both in the middle and at end
        random_binary_data[BUSTUB_PAGE_SIZE / 2] = 0;
        random_binary_data[BUSTUB_PAGE_SIZE - 1] = 0;

        // Scenario: Once we have a page, we should be able to read and write content.
        page0.with_write(|u| u.get_data_mut().copy_from_slice(&random_binary_data));
        page0.with_read(|u| assert_eq!(u.get_data(), random_binary_data.as_slice()));

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for _ in 1..buffer_pool_size {
            bpm.new_page().expect("Should be able to create new page");
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for _ in buffer_pool_size..buffer_pool_size * 2 {
            assert_eq!(bpm.new_page(), Err(NoAvailableFrameFound.into()));
        }

        // Scenario: After unpinning pages {0, 1, 2, 3, 4}, we should be able to create 5 new pages
        for i in 0..5 {
            assert_eq!(bpm.unpin_page(i, true, AccessType::default()), true, "Failed to unpin page {}", i);

            bpm.flush_page(i);
        }

        for _ in 0..5 {
            let page = bpm.new_page().expect("Must be able to create a new page");
            let page_id = page.with_read(|u| u.get_page_id());

            // Unpin the page here to allow future fetching
            bpm.unpin_page(page_id, false, AccessType::default());
        }

        // Scenario: We should be able to fetch the data we wrote a while ago.
        let page0 = bpm.fetch_page(0, AccessType::default())
            .expect("We should be able to fetch the data we wrote a while ago");

        // EXPECT_EQ(0, memcmp(page0->GetData(), random_binary_data, BUSTUB_PAGE_SIZE));
        page0.with_read(|u| assert_eq!(u.get_data(), random_binary_data.as_slice()));

        assert_eq!(bpm.unpin_page(0, true, AccessType::default()), true);

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
        let page0 = bpm.new_page().expect("The buffer pool is empty. We should be able to create a new page");

        assert_eq!(page0.with_read(|u| u.get_page_id()), 0);

        // Scenario: Once we have a page, we should be able to read and write content.

        let expected_data = "Hello";

        page0.with_write(|u| u.get_data_mut()[..expected_data.len()].copy_from_slice(expected_data.as_bytes()));
        // this is different from the original test that check the entire data
        page0.with_read(|u| assert_eq!(u.get_data(), &expected_data.as_bytes().align_to_page_data()));

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for _ in 1..buffer_pool_size {
            bpm.new_page().expect("Should be able to create new page");
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for _ in buffer_pool_size..buffer_pool_size * 2 {
            assert_eq!(bpm.new_page(), Err(NoAvailableFrameFound.into()));
        }

        // Scenario: After unpinning pages {0, 1, 2, 3, 4} and pinning another 4 new pages,
        // there would still be one buffer page left for reading page 0.
        for i in 0..5 {
            assert_eq!(bpm.unpin_page(i, true, AccessType::default()), true, "Failed to unpin page {}", i);
        }

        for _ in 0..4 {
            bpm.new_page().expect("Should be able to create a new page");
        }

        // Scenario: We should be able to fetch the data we wrote a while ago.
        let page0 = bpm.fetch_page(0, AccessType::default()).expect("should be able to fetch the data we wrote a while ago");
        page0.with_read(|u| assert_eq!(u.get_data(), &"Hello".as_bytes().align_to_page_data()));

        // Scenario: If we unpin page 0 and then make a new page, all the buffer pages should
        // now be pinned. Fetching page 0 again should fail.
        assert!(bpm.unpin_page(0, true, AccessType::default()));
        bpm.new_page().expect("Should create new page");
        assert_eq!(bpm.fetch_page(0, AccessType::default()), Err(NoAvailableFrameFound.into()));

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
        let bpm = Arc::new(BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), Some(k), None));

        let created_page_id = bpm.new_page().expect("should create new page").with_read(|u| u.get_page_id());

        let wait_for_next = Arc::new(AtomicUsize::new(0));

        let wait_for_next_thread_1 = wait_for_next.clone();
        let wait_for_next_thread_2 = wait_for_next.clone();
        let bpm_thread_1 = bpm.clone();

        // Thread 1
        let thread_1 = thread::spawn(move || {
            // Fetch the created page with write lock
            let fetch_page_with_write = bpm_thread_1.fetch_page_write(created_page_id).expect("Should be able to fetch page");

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
            let _ = bpm_thread_1.new_page().expect("Should be able to create page");

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
            let _ = bpm_thread_2.fetch_page_write(created_page_id).expect("Should be able to fetch page");

            wait_for_next_thread_2.store(3, Ordering::SeqCst);
        });

        thread_1.join().unwrap();
        thread_2.join().unwrap();

        println!("{}", wait_for_next.load(Ordering::SeqCst))
    }

    // TODO - add test that 2 threads request the same page and should only fetch once and they point to the same page
}
