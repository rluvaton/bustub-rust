use stats::{CreateTableOfStatistics, RunningTimeStats};
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct BufferPoolManagerStats {
    pub(crate) holding_inner_latch: RunningTimeStats,
    pub(crate) waiting_for_inner_latch: RunningTimeStats,
}

impl Default for BufferPoolManagerStats {
    fn default() -> Self {
        Self {
            holding_inner_latch: RunningTimeStats::new("Holding inner latch".to_string()),
            waiting_for_inner_latch: RunningTimeStats::new("Waiting for inner latch".to_string()),
        }
    }
}

impl Display for BufferPoolManagerStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferPoolManagerStats:\n\n")?;

        let table = [
            &self.holding_inner_latch,
            &self.waiting_for_inner_latch,
        ].create_table();

        f.write_str(table.to_string().as_str())
    }
}
