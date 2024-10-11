use stats::{CreateTableOfStatistics, RunningTimeStats};
use std::fmt::{Display, Formatter, Write};

#[derive(Clone)]
struct PageLatchStats {
    // TODO - can change for number of read latch request, time waiting and time holding
    pub(crate) holding_read_latch: RunningTimeStats,
    pub(crate) holding_write_latch: RunningTimeStats,
    pub(crate) waiting_for_read_latch: RunningTimeStats,
    pub(crate) waiting_for_write_latch: RunningTimeStats,
}

impl PageLatchStats {
    fn new(name: &'static str) -> Self {
        Self {
            // This is hard coded
            holding_read_latch: RunningTimeStats::new(format!("Holding {name} read latch")),
            holding_write_latch: RunningTimeStats::new(format!("Holding {name} write latch")),
            waiting_for_read_latch: RunningTimeStats::new(format!("Waiting for {name} read latch")),
            waiting_for_write_latch: RunningTimeStats::new(format!("Waiting for {name} write latch")),
        }
    }
}

#[derive(Clone)]
pub struct DiskHashTableStats {
    pub(crate) header: PageLatchStats,
    pub(crate) directory: PageLatchStats,
    pub(crate) bucket: PageLatchStats,

    pub(crate) bucket_global_split: RunningTimeStats,
    pub(crate) bucket_local_split: RunningTimeStats,

    pub(crate) bucket_global_merge: RunningTimeStats,
    pub(crate) bucket_local_merge: RunningTimeStats,
}

impl Default for DiskHashTableStats {
    fn default() -> Self {
        Self {
            header: PageLatchStats::new("header"),
            directory: PageLatchStats::new("directory"),
            bucket: PageLatchStats::new("bucket"),

            bucket_global_split: RunningTimeStats::new("bucket global split".to_string()),
            bucket_local_split: RunningTimeStats::new("bucket local split".to_string()),

            bucket_global_merge: RunningTimeStats::new("bucket global split".to_string()),
            bucket_local_merge: RunningTimeStats::new("bucket local split".to_string()),
        }
    }
}

impl Display for DiskHashTableStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiskHashTableStats:\n\n")?;

        let table = vec![
            Some(&self.header.waiting_for_read_latch),
            Some(&self.header.waiting_for_write_latch),
            Some(&self.header.holding_read_latch),
            Some(&self.header.holding_write_latch),
            None,
            Some(&self.directory.waiting_for_read_latch),
            Some(&self.directory.waiting_for_write_latch),
            Some(&self.directory.holding_read_latch),
            Some(&self.directory.holding_write_latch),
            None,
            Some(&self.bucket.waiting_for_read_latch),
            Some(&self.bucket.waiting_for_write_latch),
            Some(&self.bucket.holding_read_latch),
            Some(&self.bucket.holding_write_latch),
        ].create_table();

        f.write_str(table.to_string().as_str())?;

        f.write_str("\n")?;

        f.write_str("Bucket split and merge:\n")?;

        let table = [
            &self.bucket_global_split,
            &self.bucket_local_split,
            &self.bucket_global_merge,
            &self.bucket_local_merge,
        ].create_table();

        f.write_str(table.to_string().as_str())


    }
}
