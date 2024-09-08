use std::collections::LinkedList;
use std::sync::Arc;
use parking_lot::{Condvar, Mutex};
use common::config::{TxnId, INVALID_TXN_ID};
use crate::concurrency::LockRequest;

pub struct LockRequestQueue {

    /// List of lock requests for the same resource (table or row)
    request_queue: LinkedList<Arc<LockRequest>>,

    /// For notifying blocked transactions on this RID
    /// TODO - can replace with channel between threads?
    cv: Condvar,

    /// txn_id of an upgrading transaction (if any)
    /// Default: `INVALID_TXN_ID`
    upgrading: TxnId,

    /// coordination
    /// TODO - on what this coordination?
    latch: Mutex<()>
}

impl LockRequestQueue {
    pub fn new() -> Self {
        Self {
            request_queue: LinkedList::new(),
            cv: Condvar::new(),
            upgrading: INVALID_TXN_ID,
            latch: Mutex::new(()),
        }
    }
}


