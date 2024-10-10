#[cfg(test)]
mod tests {
    use crate::*;
    use common::{Promise, SharedFuture};
    use pages::{AlignToPageData, Page, PageId};
    use parking_lot::Mutex;
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn schedule_write_read() {
        let last_page_string = "some string to write";
        let page1 = Page::new(1);
        {
            let mut underlying_page_data = page1.write();

            underlying_page_data.get_data_mut()[0..last_page_string.len()].copy_from_slice(last_page_string.as_bytes());
        }

        let page2 = Page::new(2);
        {
            let mut underlying_page_data = page2.write();

            let val = "This is another string";
            underlying_page_data.get_data_mut()[0..val.len()].copy_from_slice(val.as_bytes());
        }

        let dm = DiskManagerUnlimitedMemory::new();
        let mut disk_scheduler = DiskScheduler::new(Arc::new(Mutex::new(dm)));

        // Write first page
        {
            let guard = page1.read();

            let mut finish_writing_future = disk_scheduler.schedule_write_page_to_disk(guard.get_page_id(), &guard);

            assert!(finish_writing_future.wait());
        }

        let mut write_second_page_future = {
            let guard = page2.read();

            disk_scheduler.schedule_write_page_to_disk(guard.get_page_id(), guard.deref())
        };

        let mut read_first_page_to_second_page_future = {
            let page1_id = page1.read().get_page_id();

            disk_scheduler.schedule_read_page_from_disk(page1_id, page2.write().deref_mut())
        };

        assert!(write_second_page_future.wait());
        assert!(read_first_page_to_second_page_future.wait());

        assert_eq!(page1.read().get_data(), &last_page_string.align_to_page_data());
        assert_eq!(page1.read().get_data(), page2.read().get_data());
    }

    #[test]
    fn should_not_handle_2_requests_at_the_same_time() {
        struct ManualDiskManager {
            map: HashMap<PageId, Promise<bool>>,
            order_of_page_id_calls: Vec<PageId>,
            order_of_page_id_finish: Vec<PageId>,
        }

        impl Default for ManualDiskManager {
            fn default() -> Self {
                ManualDiskManager {
                    map: HashMap::new(),
                    order_of_page_id_calls: vec![],
                    order_of_page_id_finish: vec![],
                }
            }
        }

        impl ManualDiskManager {
            fn get_current_queries(&self) -> Vec<PageId> {
                self.map.keys().into_iter().map(|p| p.clone()).collect()
            }
        }

        impl DiskManager for ManualDiskManager {
            fn shut_down(&mut self) {
                unimplemented!()
            }

            fn write_log(&mut self, _log_data: &[u8], _size: i32) {
                unimplemented!()
            }

            fn read_log(&mut self, _log_data: &mut [u8], _size: i32, _offset: i32) -> bool {
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

            fn set_flush_log_future(&mut self, _f: Option<SharedFuture<()>>) {
                unimplemented!()
            }

            fn has_flush_log_future(&self) -> bool {
                unimplemented!()
            }

            fn write_page(&mut self, page_id: PageId, _page_data: &[u8]) {
                self.order_of_page_id_calls.push(page_id);
                let promise = Promise::new();

                let future = promise.get_future();

                self.map.insert(page_id, promise);

                // If already has one, wait less so it will finish first
                let wait_time = if self.map.len() == 1 { 100 } else { 50 };
                future.wait_for(Duration::from_millis(wait_time));

                self.order_of_page_id_finish.push(page_id);

                self.map.remove(&page_id);
            }

            fn read_page(&mut self, page_id: PageId, _page_data: &mut [u8]) {
                self.order_of_page_id_calls.push(page_id);
                let promise = Promise::new();

                let future = promise.get_future();

                self.map.insert(page_id, promise);

                // If already has one, wait less so it will finish first
                let wait_time = if self.map.len() == 1 { 100 } else { 50 };
                future.wait_for(Duration::from_millis(wait_time));

                self.order_of_page_id_finish.push(page_id);
                self.map.remove(&page_id);
            }
        }

        let manual_manager = Arc::new(Mutex::new(ManualDiskManager::default()));
        let mut disk_scheduler = DiskScheduler::new(Arc::clone(&manual_manager));

        let page0 = Page::new(0);
        let page1 = Page::new(1);

        let mut write_page_0_future = {
            let guard = page0.read();
            disk_scheduler.schedule_write_page_to_disk(guard.get_page_id(), &guard)
        };
        let mut write_page_1_future = {
            let guard = page1.read();
            disk_scheduler.schedule_write_page_to_disk(guard.get_page_id(), &guard)
        };

        assert!(write_page_0_future.wait());
        assert!(write_page_1_future.wait());

        assert_eq!(manual_manager.lock().get_current_queries(), vec![], "everything cleaned up");
        assert_eq!(manual_manager.lock().order_of_page_id_calls, vec![0, 1], "called with the next page");
        assert_eq!(manual_manager.lock().order_of_page_id_finish, vec![0, 1], "both pages finished");

        // dm.shut_down();
    }
}
