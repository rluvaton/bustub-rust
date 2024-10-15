use std::collections::HashMap;
use std::sync::atomic::Ordering;
use common::config::{AtomicTimestamp, Timestamp};

/// Tracks all the read timestamps
pub struct Watermark {
    commit_ts: AtomicTimestamp,
    watermark: AtomicTimestamp,
    current_reads: HashMap<Timestamp, i32>,
}

impl Watermark {
    pub fn new(commit_ts: Timestamp) -> Self {
        Self {
            watermark: AtomicTimestamp::new(commit_ts),
            commit_ts: AtomicTimestamp::new(commit_ts),
            current_reads: HashMap::new(),
        }
    }

    pub fn add_txn(&self, read_ts: Timestamp) {
        assert!(read_ts >= self.commit_ts.load(Ordering::SeqCst), "read timestamp must be greater than or equal to commit timestamp");

        // TODO(fall2023): implement me!
    }

    pub fn remove_txn(&self, _read_ts: Timestamp) {
        // TODO(fall2023): implement me!
    }


    /// The caller should update commit ts before removing the txn from the watermark so that we can track watermark
    /// correctly.
    pub fn update_commit_ts(&self, commit_ts: Timestamp) {
        self.commit_ts.store(commit_ts, Ordering::SeqCst);
    }

    // TODO - should return atomic?
    pub fn get_watermark(&self) -> Timestamp {
        if self.current_reads.is_empty() {
            return self.commit_ts.load(Ordering::SeqCst);
        }

        self.watermark.load(Ordering::SeqCst)
    }
}
