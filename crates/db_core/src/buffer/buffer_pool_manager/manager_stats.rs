use std::fmt::{Display, Formatter};
use num_format::{Locale, ToFormattedString};

use stats::RunningTimeStats;
use common::Indentation;

const INDENTATION_STEP: usize = 2;

#[derive(Clone)]
pub struct BufferPoolManagerStats {
    // TODO - change this
    pub(in crate::buffer) holding_root_latch: RunningTimeStats,
    pub(in crate::buffer) waiting_for_root_latch: RunningTimeStats,
}

impl BufferPoolManagerStats {
    fn format_single_running_time_stats(stats: &RunningTimeStats, f: &mut Formatter<'_>, indentation: usize) -> std::fmt::Result {
        f.write_str(format!("{}:\n", stats.get_name()).indent(indentation).as_str())?;
        f.write_str(format!("total time: {:?}\n", stats.get_total_time()).indent(indentation + INDENTATION_STEP).as_str())?;
        f.write_str(format!("total calls: {}\n", stats.get_number_of_runs().to_formatted_string(&Locale::en)).indent(indentation + INDENTATION_STEP).as_str())?;
        f.write_str(format!("average time: {:?}", stats.calculate_average()).indent(indentation + INDENTATION_STEP).as_str())
    }
}

impl Default for BufferPoolManagerStats {
    fn default() -> Self {
        Self {
            holding_root_latch: RunningTimeStats::new("Holding root latch"),
            waiting_for_root_latch: RunningTimeStats::new("Waiting for root latch"),
        }
    }
}

impl Display for BufferPoolManagerStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BufferPoolManagerStats:\n\n")?;

        BufferPoolManagerStats::format_single_running_time_stats(&self.holding_root_latch, f, INDENTATION_STEP)?;
        write!(f, "\n\n")?;
        BufferPoolManagerStats::format_single_running_time_stats(&self.waiting_for_root_latch, f, INDENTATION_STEP)
    }
}
