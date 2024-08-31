use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub(crate) enum DurationType {
    TimeAsMilliseconds(u64),

    #[allow(unused)]
    Iteration(usize)
}

impl Default for DurationType {
    fn default() -> Self {
        DurationType::TimeAsMilliseconds(2000)
    }
}

impl DurationType {
    pub(crate) fn create_runner(&self) -> Box<dyn DurationTypeRunner> {
        match self {
            DurationType::TimeAsMilliseconds(ms) => Box::new(TimeAsMillisecondsDurationTypeRunner::new(*ms)),
            DurationType::Iteration(iteration) => Box::new(IterationDurationTypeRunner::new(*iteration))
        }
    }
}

pub(crate) trait DurationTypeRunner {
    fn should_finish(&mut self) -> bool;
}

pub(crate) struct TimeAsMillisecondsDurationTypeRunner {
    start_time: u64,
    duration_ms: u64,
}

impl TimeAsMillisecondsDurationTypeRunner {
    pub(crate) fn new(duration_ms: u64) -> Self {
        Self {
            duration_ms,
            start_time: Self::clock_ms(),
        }
    }

    fn clock_ms() -> u64 {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        duration.as_secs() * 1000 + duration.subsec_millis() as u64
    }
}

impl DurationTypeRunner for TimeAsMillisecondsDurationTypeRunner {
    fn should_finish(&mut self) -> bool {
        let now = Self::clock_ms();

        now - self.start_time > self.duration_ms
    }
}

pub(crate) struct IterationDurationTypeRunner {
    iteration_count: usize,
    current_iteration: usize
}

impl IterationDurationTypeRunner {
    pub(crate) fn new(iteration_count: usize) -> Self {
        Self {
            iteration_count,
            current_iteration: 0,
        }
    }
}

impl DurationTypeRunner for IterationDurationTypeRunner {
    fn should_finish(&mut self) -> bool {
        let should_finish = self.current_iteration >= self.iteration_count;

        if !should_finish {
            self.current_iteration += 1;
        }

        should_finish
    }
}
