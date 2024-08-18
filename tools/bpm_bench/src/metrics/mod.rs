use std::time::{SystemTime, UNIX_EPOCH};

pub mod bpm_total_metrics;
pub mod bpm_metrics;

pub fn clock_ms() -> u64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration.as_secs() * 1000 + duration.subsec_millis() as u64
}
