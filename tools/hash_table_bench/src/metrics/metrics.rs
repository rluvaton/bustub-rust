use crate::metrics::clock_ms;

pub struct Metrics {
    start_time: u64,
    last_report_at: u64,
    last_count: u64,
    pub count: u64,
    reporter: String,
    duration_ms: u64,
}

impl Metrics {
    pub fn new(reporter: String, duration_ms: u64) -> Self {
        Metrics {
            start_time: 0,
            last_report_at: 0,
            last_count: 0,
            count: 0,
            reporter,
            duration_ms,
        }
    }

    pub fn tick(&mut self) {
        self.count += 1;
    }

    pub fn begin(&mut self) {
        self.start_time = clock_ms();
    }

    pub fn report(&mut self) {
        let now = clock_ms();
        let elapsed = now - self.start_time;
        if elapsed - self.last_report_at <= 1000 {
            return;
        }
        let throughput = (self.count - self.last_count) as f64
            / (elapsed - self.last_report_at) as f64 * 1000.0;
        let avg_throughput = self.count as f64 / elapsed as f64 * 1000.0;

        println!(
            "[{:5.2}] {}: total_cnt={:<10} throughput={:<10.3} avg_throughput={:<10.3}",
            elapsed as f64 / 1000.0,
            self.reporter,
            self.count,
            throughput,
            avg_throughput
        );

        self.last_report_at = elapsed;
        self.last_count = self.count;
    }

    pub fn should_finish(&self) -> bool {
        let now = clock_ms();

        now - self.start_time > self.duration_ms
    }
}

