#[cfg(test)]
mod tests {
    use crate::disk::disk_manager::DiskManagerUnlimitedMemory;
    use crate::disk::disk_scheduler::disk_request::{ReadDiskRequest, WriteDiskRequest};
    use crate::disk::disk_scheduler::disk_scheduler::DiskScheduler;
    use common::config::BUSTUB_PAGE_SIZE;
    use common::{Promise, UnsafeSingleReferenceReadData, UnsafeSingleReferenceWriteData};
    use std::sync::{Arc};
    use parking_lot::Mutex;

    #[test]
    fn schedule_write_read() {
        let mut buf = [0u8; BUSTUB_PAGE_SIZE];
        let mut data = [0u8; BUSTUB_PAGE_SIZE];

        let dm = DiskManagerUnlimitedMemory::new();

        let mut disk_scheduler = DiskScheduler::new(Arc::new(Mutex::new(dm)));

        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        let promise1 = Promise::new();
        let future1 = promise1.get_future();
        let promise2 = disk_scheduler.create_promise();
        let future2 = promise2.get_future();

        let data_ref = unsafe { UnsafeSingleReferenceReadData::new(&data) };
        let buf_ref = unsafe { UnsafeSingleReferenceWriteData::new(&mut buf) };

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
}
