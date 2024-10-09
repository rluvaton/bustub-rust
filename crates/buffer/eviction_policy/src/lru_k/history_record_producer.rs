use std::fmt::Debug;

// Global counter
// Not using timestamp as it is slower
pub(super) type HistoryRecord = i64;

pub(super) type HistoryRecordProducer = i64;

/// History Access producer
pub(super) trait HistoryRecordProducerExt: Sized + Debug {
    type Record;

    /// Initialize the producer
    #[must_use]
    fn init() -> Self;

    // produce next value
    fn next(&mut self) -> Self::Record;
}

impl HistoryRecordProducerExt for i64 {
    type Record = Self;

    #[inline(always)]
    fn init() -> Self {
        0
    }

    #[inline(always)]
    fn next(&mut self) -> Self::Record {
        *self += 1;

        *self
    }
}

