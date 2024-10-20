use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use num_format::{Locale, ToFormattedString};
use comfy_table::{Table};
use crate::traits::CreateTableOfStatistics;

/// TODO - should rename to high precision and add unsafe to it so the consumer should make sure not using
///        methods that clone or calculate things while the running time is running
///        And have another running time stats that is safe and correct - use locks
///
/// TODO - should add running recordings in progress to disallow that?
pub struct RunningTimeStats {
    name: String,
    number_of_runs: AtomicU64,

    /// Total duration in nanoseconds
    total_duration: AtomicU64,
}

pub struct SingleRun<'a> {
    start_time: Instant,

    /// For adding
    run_time: &'a RunningTimeStats,
}

impl RunningTimeStats {
    pub fn new(name: String) -> Self {
        Self {
            name,
            number_of_runs: AtomicU64::new(0),
            total_duration: AtomicU64::new(0),
        }
    }

    pub fn create_single(&self) -> SingleRun {
        SingleRun {
            start_time: Instant::now(),
            run_time: &self,
        }
    }

    pub(crate) fn record_new_run(&self, nano_seconds_time: u64) {
        self.number_of_runs.fetch_add(1, Ordering::SeqCst);
        self.total_duration.fetch_add(nano_seconds_time, Ordering::SeqCst);
    }

    /// This should be done after finishing adding all data for correct result
    pub fn calculate_average(&self) -> Duration {
        let duration = self.total_duration.load(Ordering::SeqCst);
        let number_of_runs = self.number_of_runs.load(Ordering::SeqCst);

        if number_of_runs == 0 {
            return Duration::ZERO;
        }

        Duration::from_nanos(duration / number_of_runs)
    }

    /// This should be done after finishing adding all data for correct result
    pub fn get_total_time(&self) -> Duration {
        Duration::from_nanos(self.total_duration.load(Ordering::SeqCst))
    }

    /// This should be done after finishing adding all data for correct result
    pub fn get_number_of_runs(&self) -> u64 {
        self.number_of_runs.load(Ordering::SeqCst)
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for RunningTimeStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: Average {:?}", self.name, self.calculate_average())
    }
}

impl CreateTableOfStatistics for [&RunningTimeStats] {
    fn create_table(&self) -> Table {
        self
            .iter()
            .map(|&item| Some(item))
            .collect::<Vec<Option<&RunningTimeStats>>>()
            .create_table()
    }
}

// None will act as an empty row
impl CreateTableOfStatistics for Vec<Option<&RunningTimeStats>> {
    fn create_table(&self) -> Table {
        let mut table = Table::new();

        table.set_header(vec!["name", "total time", "total calls", "average time"]);

        for &stat in self {
            if let Some(stat) = stat {
                table.add_row(
                    vec![
                    stat.name.clone(),
                    format!("{:?}", stat.get_total_time()),
                    stat.get_number_of_runs().to_formatted_string(&Locale::en),
                    format!("{:?}", stat.calculate_average())
                ]
                );
            } else {
                table.add_row::<Vec<_>>(vec!["", "", "", ""]);
            }
        }

        table
    }
}

impl Clone for RunningTimeStats {
    /// Should not be cloned while keep adding new to duration
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            total_duration: AtomicU64::new(self.total_duration.load(Ordering::SeqCst)),
            number_of_runs: AtomicU64::new(self.number_of_runs.load(Ordering::SeqCst)),
        }
    }
}

impl Drop for SingleRun<'_> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();

        // THE duration will not exceed u64 size
        self.run_time.record_new_run(duration.as_nanos() as u64)
    }
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, Instant};
    use crate::running_time_stats::RunningTimeStats;


    /// Using this over `thread::sleep` for better accuracy
    fn block_sleep_nanoseconds(ns: u128) {
        let start = Instant::now();

        while start.elapsed().as_nanos() < ns {
            // nothing
        }
    }

    /// Using this over `thread::sleep` for better accuracy
    fn block_sleep_microseconds(microseconds: u128) {
        block_sleep_nanoseconds(microseconds * 1000);
    }

    #[test]
    fn run_time_single_average_should_be_correct() {
        let r = RunningTimeStats::new("test".to_string());

        {
            let _s = r.create_single();
            thread::sleep(Duration::from_millis(1));
        }

        let average = r.calculate_average();


        assert!(average.as_micros() < 2000, "average microseconds (µs) should be less than 2000µs (2ms) {:?}", average);
        assert!(average.as_micros() > 800, "average microseconds (µs) should be greater than 800µs (0.8ms) {:?}", average);
    }

    #[test]
    fn run_time_average_for_10_ms_should_be_around_10_ms() {
        let r = RunningTimeStats::new("test".to_string());

        {
            let _s = r.create_single();
            thread::sleep(Duration::from_millis(10));
        }

        let average = r.calculate_average();

        assert!(average.as_millis() < 20, "average ms should be less than 20ms {:?}", average);
        assert!(average.as_millis() > 8, "average ms should be greater than 8ms {:?}", average);
    }

    #[test]
    fn run_time_three_average_should_be_correct() {
        let r = RunningTimeStats::new("test".to_string());

        {
            let _s = r.create_single();

            // 1.5ms
            thread::sleep(Duration::from_micros(1500));
        }
        {
            let _s = r.create_single();

            // 1ms
            thread::sleep(Duration::from_micros(1000));
        }
        {
            let _s = r.create_single();

            // 0.5ms
            thread::sleep(Duration::from_micros(500));
        }

        let average = r.calculate_average();


        assert!(average.as_micros() < 2000, "average microseconds (µs) should be less than 2000µs (2ms) {:?}", average);
        assert!(average.as_micros() > 800, "average microseconds (µs) should be greater than 800µs (0.8ms) {:?}", average);
    }

    #[test]
    fn run_time_three_average_should_be_correct_when_in_microseconds() {
        let r = RunningTimeStats::new("test".to_string());

        {
            let _s = r.create_single();

            block_sleep_microseconds(150);
        }
        {
            let _s = r.create_single();

            block_sleep_microseconds(100);
        }
        {
            let _s = r.create_single();

            block_sleep_microseconds(50);
        }

        let average = r.calculate_average();


        assert!(average.as_micros() < 150, "average microseconds (µs) should be less than 150µs {:?}", average);
        assert!(average.as_micros() > 50, "average microseconds (µs) should be greater than 50µs {:?}", average);
    }


    /// We can't check nanoseconds as this will depend on the machine
    #[ignore]
    #[test]
    fn run_time_three_average_should_be_correct_when_in_nanoseconds() {
        let r = RunningTimeStats::new("test".to_string());

        {
            let _s = r.create_single();

            block_sleep_nanoseconds(150);
        }
        {
            let _s = r.create_single();

            block_sleep_nanoseconds(100);
        }
        {
            let _s = r.create_single();

            block_sleep_nanoseconds(50);
        }

        let average = r.calculate_average();


        assert!(average.as_nanos() < 150, "average nanoseconds (ns) should be less than 150ns {:?}", average);
        assert!(average.as_nanos() > 50, "average nanoseconds (ns) should be greater than 50ns {:?}", average);
    }
}
