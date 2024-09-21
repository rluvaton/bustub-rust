#[cfg(test)]
mod tests {
    use crate::buffer::{PinPageGuard, BufferPoolManager};
    use parking_lot::Mutex;
    use std::sync::Arc;
    use common::config::BUSTUB_PAGE_SIZE;
    use crate::storage::{DiskManager, DiskManagerUnlimitedMemory};

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
            let _page2 = buffer_pool_manager.new_page_guarded().expect("Should be able to create a page");
        }

        disk_manager.lock().shut_down();
    }

    #[test]
    fn casting_and_dropping() {
        // this should not compile
        todo!()
        // #[repr(C)]
        // struct Hello {
        //     key: u32,
        //     rest: [u8; BUSTUB_PAGE_SIZE - 4],
        // }
        // let buffer_pool_size = 5;
        // let k = 2;
        //
        // let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));
        //
        // let buffer_pool_manager = Arc::new(BufferPoolManager::new(
        //     buffer_pool_size,
        //     Arc::clone(&disk_manager),
        //     Some(k),
        //     None,
        // ));
        //
        // let mut page0 = buffer_pool_manager.new_page_write_guarded().expect("should be able to create a page");
        //
        // let data = page0.cast_mut::<Hello>();
        //
        // data.key = 10;
        // data.rest = [3; BUSTUB_PAGE_SIZE - 4];
        //
        // drop(page0);
        //
        // assert_eq!(data.key, 10);
    }
}
