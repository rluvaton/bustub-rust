use std::collections::HashMap;
use common::config::Timestamp;

/// Tracks all the read timestamps
pub struct Watermark {
    commit_ts: Timestamp,
    watermark: Timestamp,
    current_reads: HashMap<Timestamp, i32>,
}

impl Watermark {
    pub fn new(commit_ts: Timestamp) -> Self {
        Self {
            watermark: commit_ts,
            commit_ts,
            current_reads: HashMap::new(),
        }
    }

    pub fn add_txn(&mut self, read_ts: Timestamp) {
        assert!(read_ts < self.commit_ts, "read timestamp must be greater than or equal to commit timestamp");

        // TODO(fall2023): implement me!
        todo!()
    }

    pub fn remove_txn(&mut self, read_ts: Timestamp) {
        // TODO(fall2023): implement me!
        todo!()
    }


    /// The caller should update commit ts before removing the txn from the watermark so that we can track watermark
    /// correctly.
    pub fn update_commit_ts(&mut self, commit_ts: Timestamp) {
        self.commit_ts = commit_ts;
    }

    pub fn get_watermark(&self) -> Timestamp {
        if self.current_reads.is_empty() {
            return self.commit_ts;
        }

        self.watermark
    }
}
