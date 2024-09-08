use crate::concurrency::UndoLink;
use crate::storage::Tuple;
use common::config::{Timestamp, INVALID_TIMESTAMP};

#[derive(Clone)]
pub struct UndoLog {
    /// Whether this log is a deletion marker
    is_deleted: bool,

    /// The fields modified by this undo log
    modified_fields: Vec<bool>,

    /// The modified fields
    tuple: Tuple,

    /// Timestamp of this undo log
    /// Default: INVALID_TIMESTAMP
    ts: Timestamp,

    /// Undo log prev version
    /// Default:
    prev_version: UndoLink,
}

impl UndoLog {
    pub fn new(is_deleted: bool, modified_fields: Vec<bool>, tuple: Tuple, ts: Option<Timestamp>, prev_version: Option<UndoLink>) -> Self {
        Self {
            is_deleted,
            modified_fields,
            tuple,
            ts: ts.unwrap_or(INVALID_TIMESTAMP),
            prev_version: prev_version.unwrap_or(UndoLink::default())
        }
    }
}

