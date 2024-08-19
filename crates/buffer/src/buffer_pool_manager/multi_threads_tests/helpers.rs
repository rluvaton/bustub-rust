use std::time::{SystemTime, UNIX_EPOCH};
use tempdir::TempDir;

pub(crate) fn get_tmp_dir() -> TempDir {
    TempDir::new("buffer_pool_manager_multi_threads_tests").expect("Should create tmp directory")
}

pub(crate) struct RunTimer {
    start_time: u64,
    duration_ms: u64,
}

impl RunTimer {
    pub(crate) fn new(duration_ms: u64) -> RunTimer {
        RunTimer {
            duration_ms,
            start_time: Self::clock_ms(),
        }
    }

    pub(crate) fn should_finish(&self) -> bool {
        let now = Self::clock_ms();

        now - self.start_time > self.duration_ms
    }

    fn clock_ms() -> u64 {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        duration.as_secs() * 1000 + duration.subsec_millis() as u64
    }
}
