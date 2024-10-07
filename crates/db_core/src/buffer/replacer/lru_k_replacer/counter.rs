use std::sync::atomic::{AtomicI64, Ordering};

#[derive(Debug)]
pub(in crate::buffer) struct AtomicI64Counter(AtomicI64);

impl AtomicI64Counter {
    pub(in crate::buffer) fn new(initial_value: i64) -> Self {
        AtomicI64Counter(AtomicI64::new(initial_value))
    }

    #[inline]
    pub(in crate::buffer) fn get_next(&self) -> i64 {
        // Take the current value and increment
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for AtomicI64Counter {
    fn default() -> Self {
        Self::new(0)
    }
}
