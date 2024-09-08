use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub(in crate::buffer) struct AtomicU64Counter(AtomicU64);

impl AtomicU64Counter {
    pub(in crate::buffer) fn new(initial_value: u64) -> Self {
        AtomicU64Counter(AtomicU64::new(initial_value))
    }

    pub(in crate::buffer) fn get_next(&self) -> u64 {
        // Take the current value and increment
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for AtomicU64Counter {
    fn default() -> Self {
        Self::new(0)
    }
}
