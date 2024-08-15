#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::disk::disk_manager::DiskManagerUnlimitedMemory;
    use crate::disk::disk_scheduler::disk_request::{ReadDiskRequest, WriteDiskRequest};
    use crate::disk::disk_scheduler::disk_scheduler::DiskScheduler;
    use common::config::BUSTUB_PAGE_SIZE;
    use common::Promise;
    use std::sync::{Arc};
    use parking_lot::Mutex;

    #[test]
    fn schedule_write_read() {
        let mut buf = [0u8; BUSTUB_PAGE_SIZE as usize];
        let mut data = [0u8; BUSTUB_PAGE_SIZE as usize];

        let dm = DiskManagerUnlimitedMemory::new();

        let mut disk_scheduler = DiskScheduler::new(Arc::new(Mutex::new(dm)));

        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        let promise1 = Promise::new();
        let future1 = promise1.get_future();
        let promise2 = disk_scheduler.create_promise();
        let future2 = promise2.get_future();

        let data = Arc::new(Box::from(data));
        let buf = Arc::new(Mutex::new(Box::from(buf)));

        disk_scheduler.schedule(
            WriteDiskRequest {
                data: Arc::clone(&data),
                page_id: 0,
                callback: promise1
            }.into(),
        );
        disk_scheduler.schedule(
            ReadDiskRequest {
                data: Arc::clone(&buf),

                page_id: 0,
                callback: promise2
            }.into(),
        );

        assert!(future1.wait());
        assert!(future2.wait());

        let buf_ref = buf.clone();
        let buf_data = buf_ref.lock();

        assert_eq!(buf_data.deref(), data.deref());

        // dm.shut_down();
    }
}
