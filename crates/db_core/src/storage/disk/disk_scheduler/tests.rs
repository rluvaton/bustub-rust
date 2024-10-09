#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use pages::{PageId, PAGE_SIZE};
    use common::{Future, Promise, UnsafeSingleRefData, UnsafeSingleRefMutData};
    use std::sync::{Arc};
    use std::time::Duration;
    use parking_lot::Mutex;
    use crate::storage::*;

    #[test]
    fn schedule_write_read() {
        let mut buf = [0u8; PAGE_SIZE];
        let mut data = [0u8; PAGE_SIZE];

        let dm = DiskManagerUnlimitedMemory::new();

        let mut disk_scheduler = DiskScheduler::new(Arc::new(Mutex::new(dm)));

        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        let promise1 = Promise::new();
        let future1 = promise1.get_future();
        let promise2 = disk_scheduler.create_promise();
        let future2 = promise2.get_future();

        let data_ref = unsafe { UnsafeSingleRefData::new(&data) };
        let buf_ref = unsafe { UnsafeSingleRefMutData::new(&mut buf) };

        disk_scheduler.schedule(
            WriteDiskRequest {
                data: data_ref,
                page_id: 0,
                callback: promise1
            }.into(),
        );
        disk_scheduler.schedule(
            ReadDiskRequest {
                data: buf_ref,

                page_id: 0,
                callback: promise2
            }.into(),
        );

        assert!(future1.wait());
        assert!(future2.wait());

        assert_eq!(buf, data);

        // dm.shut_down();
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

            fn set_flush_log_future(&mut self, _f: Option<Future<()>>) {
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

        let data1 = [0u8; PAGE_SIZE];
        let data2 = [0u8; PAGE_SIZE];

        let promise1 = Promise::new();
        let future1 = promise1.get_future();

        let promise2 = Promise::new();
        let future2 = promise2.get_future();

        let data1_ref = unsafe { UnsafeSingleRefData::new(&data1) };
        let data2_ref = unsafe { UnsafeSingleRefData::new(&data2) };

        disk_scheduler.schedule(
            WriteDiskRequest {
                data: data1_ref,
                page_id: 0,
                callback: promise1
            }.into(),
        );
        disk_scheduler.schedule(
            WriteDiskRequest {
                data: data2_ref,
                page_id: 1,
                callback: promise2
            }.into(),
        );

        assert!(future1.wait());
        assert!(future2.wait());

        assert_eq!(manual_manager.lock().get_current_queries(), vec![], "everything cleaned up");
        assert_eq!(manual_manager.lock().order_of_page_id_calls, vec![0, 1], "called with the next page");
        assert_eq!(manual_manager.lock().order_of_page_id_finish, vec![0, 1], "both pages finished");

        // dm.shut_down();
    }
}
