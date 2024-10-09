use crate::UndoLink;
use tuple::Tuple;
use common::config::{Timestamp, INVALID_TIMESTAMP};

#[derive(Clone)]
pub struct UndoLog {
    /// Whether this log is a deletion marker
    #[allow(unused)]
    is_deleted: bool,

    /// The fields modified by this undo log
    #[allow(unused)]
    modified_fields: Vec<bool>,

    /// The modified fields
    #[allow(unused)]
    tuple: Tuple,

    /// Timestamp of this undo log
    /// Default: INVALID_TIMESTAMP
    #[allow(unused)]
    ts: Timestamp,

    /// Undo log prev version
    /// Default:
    #[allow(unused)]
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

