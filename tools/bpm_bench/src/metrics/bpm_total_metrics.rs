use std::fmt::{Display, Formatter};
use parking_lot::Mutex;
use crate::metrics::clock_ms;

struct Metrics {
    scan_count: u64,
    get_count: u64,
}

pub struct BpmTotalMetrics {
    start_time: u64,
    metrics: Mutex<Metrics>,
}

impl BpmTotalMetrics {
    pub fn new() -> Self {
        BpmTotalMetrics {
            start_time: 0,
            metrics: Mutex::new(Metrics {
                scan_count: 0,
                get_count: 0
            }),
        }
    }

    pub fn begin(&mut self) {
        self.start_time = clock_ms();
    }

    pub fn report_scan(&self, scan_count: u64) {
        self.metrics.lock().scan_count += scan_count;
    }

    pub fn report_get(&self, get_count: u64) {
        self.metrics.lock().get_count += get_count;
    }

    pub fn report(&self) {
        println!("{}", self);
    }
}

impl Display for BpmTotalMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let now = clock_ms();

        let elapsed: f64 = (now - self.start_time) as f64;
        let metrics = self.metrics.lock();
        let scan_per_sec = (metrics.scan_count as f64) / elapsed * 1000f64;
        let get_per_sec = (metrics.get_count as f64) / elapsed * 1000f64;

        write!(f, "<<< BEGIN\nscan: {}\nget: {}\n>>> END", scan_per_sec, get_per_sec)
    }
}
