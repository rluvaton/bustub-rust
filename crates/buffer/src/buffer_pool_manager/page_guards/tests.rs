#[cfg(test)]
mod tests {
    use crate::buffer_pool_manager::PinPageGuard;
    use crate::BufferPoolManager;
    use parking_lot::Mutex;
    use std::sync::Arc;
    use storage::{DiskManager, DiskManagerUnlimitedMemory};

    #[test]
    fn sample() {
        let buffer_pool_size = 5;
        let k = 2;

        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));

        let buffer_pool_manager = Arc::new(BufferPoolManager::new(
            buffer_pool_size,
            Arc::clone(&disk_manager),
            Some(k),
            None,
        ));

        let page0 = buffer_pool_manager.new_page().expect("should be able to create a page");

        let guarded_page = PinPageGuard::new(Arc::clone(&buffer_pool_manager), page0.clone());

        {
            let page0_guard = page0.read();
            let guarder_page_guard = guarded_page.read();

            assert_eq!(page0_guard.get_data(), guarder_page_guard.get_data());
            assert_eq!(page0_guard.get_page_id(), guarder_page_guard.get_page_id());
        }

        assert_eq!(page0.get_pin_count(), 1);

        // Should decrement pin count
        drop(guarded_page);

        assert_eq!(page0.get_pin_count(), 0);

        {
            let page2 = buffer_pool_manager.new_page().expect("Should be able to create a page");
            // TODO: we acquire read latch before creating the guard?
            //     page2->RLatch()
            // Should both increment pin count and acquire read lock?
            // let guard = PinReadPageGuard::new(Arc::clone(&buffer_pool_manager), page2);
        }

        disk_manager.lock().shut_down();
    }
}
