#[cfg(test)]
mod tests {
    use crate::buffer_pool_manager::manager::BufferPoolManager;
    use crate::lru_k_replacer::AccessType;
    use common::config::BUSTUB_PAGE_SIZE;
    use parking_lot::Mutex;
    use rand::Rng;
    use std::sync::Arc;
    use storage::{DefaultDiskManager, DiskManager};

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

        let mut disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let mut bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), None, None);

        let page0 = bpm.new_page();

        // Scenario: The buffer pool is empty. We should be able to create a new page.
        assert_ne!(page0, None);

        let mut page0 = page0.unwrap();

        assert_eq!(page0.get_page_id(), 0);

        // Generate random binary data
        let mut random_binary_data: [u8; BUSTUB_PAGE_SIZE] = [0; BUSTUB_PAGE_SIZE];

        random_binary_data.fill_with(|| rng.gen_range(lower_bound..upper_bound) as u8);

        // Insert terminal characters both in the middle and at end
        random_binary_data[BUSTUB_PAGE_SIZE / 2] = 0;
        random_binary_data[BUSTUB_PAGE_SIZE - 1] = 0;

        // Scenario: Once we have a page, we should be able to read and write content.
        // std::memcpy(page0->GetData(), random_binary_data, BUSTUB_PAGE_SIZE);
        page0.get_data_mut().copy_from_slice(&random_binary_data);
        assert_eq!(page0.get_data().as_slice(), random_binary_data);

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for i in 1..buffer_pool_size {
            assert_ne!(bpm.new_page(), None);
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for i in buffer_pool_size..buffer_pool_size * 2 {
            assert_eq!(bpm.new_page(), None);
        }

        // Scenario: After unpinning pages {0, 1, 2, 3, 4}, we should be able to create 5 new pages
        for i in 0..5 {
            assert_eq!(bpm.unpin_page(i, true, AccessType::default()), true);

            bpm.flush_page(i);
        }

        for i in 0..5 {
            let page = bpm.new_page().expect("Must be able to create a new page");
            let page_id = page.get_page_id();

            // Unpin the page here to allow future fetching
            bpm.unpin_page(page_id, false, AccessType::default());
        }

        // Scenario: We should be able to fetch the data we wrote a while ago.
        let page0 = bpm.fetch_page(0, AccessType::default());
        assert_ne!(page0, None);

        // EXPECT_EQ(0, memcmp(page0->GetData(), random_binary_data, BUSTUB_PAGE_SIZE));
        assert_eq!(page0.unwrap().get_data().as_slice(), random_binary_data);
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

        let mut disk_manager = DefaultDiskManager::new(db_name).expect("should create disk manager");
        let mut bpm = BufferPoolManager::new(buffer_pool_size, Arc::new(Mutex::new(disk_manager)), None, None);

        let page0 = bpm.new_page();

        // Scenario: The buffer pool is empty. We should be able to create a new page.
        assert_ne!(page0, None);

        let mut page0 = page0.unwrap();

        assert_eq!(page0.get_page_id(), 0);

        // Scenario: Once we have a page, we should be able to read and write content.
        page0.get_data_mut().copy_from_slice("Hello".as_bytes());
        assert_eq!(page0.get_data(), "Hello".as_bytes());

        // Scenario: We should be able to create new pages until we fill up the buffer pool.
        for i in 1..buffer_pool_size {
            assert_ne!(bpm.new_page(), None);
        }

        // Scenario: Once the buffer pool is full, we should not be able to create any new pages.
        for i in buffer_pool_size..buffer_pool_size * 2 {
            assert_eq!(bpm.new_page(), None);
        }

        // Scenario: After unpinning pages {0, 1, 2, 3, 4} and pinning another 4 new pages,
        // there would still be one buffer page left for reading page 0.
        for i in 0..5 {
            assert!(bpm.unpin_page(i, true, AccessType::default()));
        }
        for i in 0..4 {
            assert_ne!(bpm.new_page(), None);
        }

        // Scenario: We should be able to fetch the data we wrote a while ago.
        let page0 = bpm.fetch_page(0, AccessType::default()).expect("should be able to fetch the data we wrote a while ago");
        assert_eq!(page0.get_data(), "Hello".as_bytes());

        // Scenario: If we unpin page 0 and then make a new page, all the buffer pages should
        // now be pinned. Fetching page 0 again should fail.
        assert!(bpm.unpin_page(0, true, AccessType::default()));
        bpm.new_page().expect("Should create new page");
        assert_eq!(bpm.fetch_page(0, AccessType::default()), None);

        // Shutdown the disk manager and remove the temporary file we created.
        // TODO - shutdown
        // disk_manager.shut_down();
        // remove("test.db");
        //
        // delete bpm;
        // delete disk_manager;
    }


}
