use crate::concurrency::UndoLink;

/// The first undo link in the version chain, that links table heap tuple to the undo log.
#[derive(Clone, PartialEq)]
pub struct VersionUndoLink {
    /// The next version in the version chain.
    prev: UndoLink,

    /// Whether a transaction is modifying the version link. Fall 2023: you do not need to read / write this field until task 4.2
    /// Default: false
    in_progress: bool
}

impl VersionUndoLink {
    pub fn new(prev: UndoLink, in_progress: bool) -> Self {
        Self {
            prev,
            in_progress
        }
    }

    pub fn from_optional_undo_link(undo_link: Option<UndoLink>) -> Option<VersionUndoLink> {
        if let Some(undo_link) = undo_link {
            Some(VersionUndoLink::new(undo_link, false))
        } else {
            None
        }

    }
}

impl From<UndoLink> for VersionUndoLink {
    fn from(value: UndoLink) -> Self {
        Self::new(value, false)
    }
}

