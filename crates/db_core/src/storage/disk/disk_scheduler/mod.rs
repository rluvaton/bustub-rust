mod disk_scheduler;
mod disk_request;
mod tests;

pub use disk_scheduler::DiskScheduler;
pub use disk_request::{ReadDiskRequest, WriteDiskRequest, DiskRequestType};
