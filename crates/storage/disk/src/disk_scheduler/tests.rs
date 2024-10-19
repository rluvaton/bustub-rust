#[cfg(test)]
mod tests {
    use crate::*;
    use common::{Future, Promise};
    use pages::{AlignToPageData, Page, PageId};
    use parking_lot::Mutex;
    use std::collections::HashMap;
    use std::ops::Deref;
    use std::sync::Arc;
    use std::time::Duration;


    #[test]
    fn schedule_write_before_read() {
        let page1_string = "some string to write";
        let page1 = Page::new(1);
        {
            let mut underlying_page_data = page1.write();

            underlying_page_data.get_data_mut()[0..page1_string.len()].copy_from_slice(page1_string.as_bytes());
        }

        let page2_string = "This is another string";
        let page2 = Page::new(2);
        {
            let mut underlying_page_data = page2.write();

            underlying_page_data.get_data_mut()[0..page2_string.len()].copy_from_slice(page2_string.as_bytes());
        }

        let dm = DiskManagerUnlimitedMemory::new();
        let disk_scheduler = Arc::new(DiskScheduler::new(Arc::new(dm)));

        // Write first page
        {
            let mut guard = page1.read();

            let finish_writing_result = disk_scheduler.clone().write_page_to_disk(
                &mut guard,
                || {},
            );

            assert_eq!(finish_writing_result, (true, ()));
        }

        {
            // write page 2 and read page 1 into page 2 buffer
            let mut guard = page2.write();

            let (finish_writing_result, _) = disk_scheduler.clone().write_and_read_page_from_disk(
                &mut guard,
                page1.read().get_page_id(),
                || {},
            );

            assert_eq!(finish_writing_result, true);
        };

        assert_eq!(page1.read().get_data(), &page1_string.align_to_page_data());
        assert_eq!(page1.read().get_data(), page2.read().get_data());

        {
            // read page 2 from disk into page 1 buffer
            let mut guard = page1.write();
            guard.set_page_id(page2.read().get_page_id());

            disk_scheduler.clone().read_page_from_disk(
                &mut guard,
                || {},
            );
        };

        assert_eq!(page1.read().get_data(), &page2_string.align_to_page_data());
    }

    #[test]
    fn should_not_handle_2_requests_at_the_same_time() {
        struct ManualInnerDiskManager {
            map: HashMap<PageId, Promise<bool>>,
            pub order_of_page_id_calls: Vec<PageId>,
            pub order_of_page_id_finish: Vec<PageId>,
        }

        struct ManualDiskManager(Mutex<ManualInnerDiskManager>);

        impl Default for ManualDiskManager {
            fn default() -> Self {
                ManualDiskManager(Mutex::new(ManualInnerDiskManager {
                    map: HashMap::new(),
                    order_of_page_id_calls: vec![],
                    order_of_page_id_finish: vec![],
                }))
            }
        }

        impl ManualDiskManager {
            fn get_current_queries(&self) -> Vec<PageId> {
                self.0.lock().map.keys().into_iter().map(|p| p.clone()).collect()
            }
        }

        impl DiskManager for ManualDiskManager {
            fn shut_down(&mut self) {
                unimplemented!()
            }

            fn write_log(&self, _log_data: &[u8], _size: i32) {
                unimplemented!()
            }

            fn read_log(&self, _log_data: &mut [u8], _size: i32, _offset: i32) -> bool {
                unimplemented!()
            }

            fn get_num_flushes(&self) -> i32 {
                unimplemented!()
            }

            fn get_flush_state(&self) -> bool {
                unimplemented!()
            }

            fn get_num_writes(&self) -> i32 {
                unimplemented!()
            }

            fn set_flush_log_future(&mut self, _f: Option<Future<()>>) {
                unimplemented!()
            }

            fn has_flush_log_future(&self) -> bool {
                unimplemented!()
            }

            fn write_page(&self, page_id: PageId, _page_data: &[u8]) {
                let mut inner = self.0.lock();
                inner.order_of_page_id_calls.push(page_id);
                let promise = Promise::new();

                let future = promise.get_future();

                inner.map.insert(page_id, promise);

                // If already has one, wait less so it will finish first
                let wait_time = if inner.map.len() == 1 { 100 } else { 50 };
                future.wait_for(Duration::from_millis(wait_time));

                inner.order_of_page_id_finish.push(page_id);

                inner.map.remove(&page_id);
            }

            fn read_page(&self, page_id: PageId, _page_data: &mut [u8]) {
                let mut inner = self.0.lock();

                inner.order_of_page_id_calls.push(page_id);
                let promise = Promise::new();

                let future = promise.get_future();

                inner.map.insert(page_id, promise);

                // If already has one, wait less so it will finish first
                let wait_time = if inner.map.len() == 1 { 100 } else { 50 };
                future.wait_for(Duration::from_millis(wait_time));

                inner.order_of_page_id_finish.push(page_id);
                inner.map.remove(&page_id);
            }
        }

        let manual_manager = Arc::new(ManualDiskManager::default());

        let mut disk_scheduler = DiskScheduler::new(Arc::clone(&manual_manager));

        let page1 = Page::new(0);
        let page2 = Page::new(1);

        let future1 = unsafe { disk_scheduler.schedule_write_page_to_disk(page1.read().deref()) };
        let future2 = unsafe { disk_scheduler.schedule_read_page_from_disk(&mut page2.write()) };

        assert!(future1.wait());
        assert!(future2.wait());

        assert_eq!(manual_manager.get_current_queries(), vec![], "everything cleaned up");
        assert_eq!(manual_manager.0.lock().order_of_page_id_calls, vec![0, 1], "called with the next page");
        assert_eq!(manual_manager.0.lock().order_of_page_id_finish, vec![0, 1], "both pages finished");

        // dm.shut_down();
    }
}
