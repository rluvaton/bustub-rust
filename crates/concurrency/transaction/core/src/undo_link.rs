use common::config::{TxnId, INVALID_TXN_ID};

/// Represents a link to a previous version of this tuple
#[derive(Clone, PartialEq)]
pub struct UndoLink {
    /// Previous version can be found in which txn
    prev_txn: TxnId,

    /// The log index of the previous version in `prev_txn`
    prev_log_idx: i32,
}

impl UndoLink {
    pub fn new(prev_txn: TxnId, prev_log_idx: i32) -> Self {
        Self {
            prev_txn,
            prev_log_idx,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.prev_txn != INVALID_TXN_ID
    }
}

impl Default for UndoLink {
    fn default() -> Self {
        Self::new(INVALID_TXN_ID, 0)
    }
}
