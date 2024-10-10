use std::fmt::{Display, Formatter};
use parking_lot::Mutex;
use crate::metrics::clock_ms;

struct Metrics {
    write_count: u64,
    read_count: u64,
}

pub struct TotalMetrics {
    start_time: u64,
    metrics: Mutex<Metrics>,
}

impl TotalMetrics {
    pub fn new() -> Self {
        TotalMetrics {
            start_time: 0,
            metrics: Mutex::new(Metrics {
                write_count: 0,
                read_count: 0
            }),
        }
    }

    pub fn begin(&mut self) {
        self.start_time = clock_ms();
    }

    pub fn report_write(&self, write_count: u64) {
        self.metrics.lock().write_count += write_count;
    }

    pub fn report_read(&self, read_count: u64) {
        self.metrics.lock().read_count += read_count;
    }

    pub fn report(&self) {
        println!("{}", self);
    }
}

impl Display for TotalMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let now = clock_ms();

        let elapsed: f64 = (now - self.start_time) as f64;
        let metrics = self.metrics.lock();
        let write_per_sec = (metrics.write_count as f64) / elapsed * 1000f64;
        let read_per_sec = (metrics.read_count as f64) / elapsed * 1000f64;

        write!(f, "<<< BEGIN\nwrite: {}\nread: {}\n>>> END", write_per_sec, read_per_sec)
    }
}
