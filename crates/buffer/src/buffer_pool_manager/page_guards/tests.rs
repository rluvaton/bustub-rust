#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use parking_lot::Mutex;
    use storage::DiskManagerUnlimitedMemory;
    use crate::buffer_pool_manager::BasicPageGuard;
    use crate::BufferPoolManager;

    #[test]
    fn sample() {
        let buffer_pool_size = 5;
        let k = 2;

        let disk_manager = Arc::new(Mutex::new(DiskManagerUnlimitedMemory::new()));

        let buffer_pool_manager = Arc::new(BufferPoolManager::new(
            buffer_pool_size,
            disk_manager,
            Some(k),
            None,
        ));

        let page0 = buffer_pool_manager.new_page().expect("should be able to create a page");

        let guarded_page = BasicPageGuard::new(Arc::clone(&buffer_pool_manager), page0);

        // TODO - continue
    }
}
