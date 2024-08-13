#[cfg(test)]
mod tests {
    use common::config::BUSTUB_PAGE_SIZE;
    use crate::disk::disk_manager::{DiskManager, DiskManagerUnlimitedMemory};
    use crate::disk::disk_scheduler::disk_request::DiskRequest;
    use crate::disk::disk_scheduler::disk_scheduler::DiskScheduler;

    #[test]
    fn schedule_write_read() {
        let mut buf = [0u8; BUSTUB_PAGE_SIZE as usize];
        let mut data = [0u8; BUSTUB_PAGE_SIZE as usize];

        let mut dm = DiskManagerUnlimitedMemory::new();

        let disk_scheduler = DiskScheduler::new(&dm);

        let val = "A test string.";
        data[0..val.len()].copy_from_slice(val.as_bytes());

        let promise1 = disk_scheduler.create_promise();
        let future1 = promise1.get_future();
        let promise2 = disk_scheduler.create_promise();
        let future2 = promise2.get_future();

        disk_scheduler.schedule(
            DiskRequest {
                is_write: true,
                data: data.as_ref(),
                page_id: 0,
                callback: promise1
            },
        );
        disk_scheduler.schedule(
            DiskRequest {
                is_write: false,
                data: buf.as_ref(),
                page_id: 0,
                callback: promise2
            },
        );

        assert!(future1.wait());
        assert!(future2.wait());

        assert_eq!(buf, data);

        dm.shut_down();
    }
}
