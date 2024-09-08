use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use parking_lot::Mutex;
use common::config::{TableOID, TxnId};
use common::RID;
use crate::concurrency::{LockRequestQueue, TransactionManager};

pub struct LockManager {
    transaction_manager: Arc<TransactionManager>,

    /// Structure that holds lock requests for a given table oid
    table_lock_map: Mutex<HashMap<TableOID, Arc<LockRequestQueue>>>,

    /** Structure that holds lock requests for a given RID */
    row_lock_map: Mutex<HashMap<RID, Arc<LockRequestQueue>>>,

    enable_cycle_detection: AtomicBool,
    cycle_detection_thread: JoinHandle<()>,

    /// Waits-for graph representation.
    waits_for: Mutex<HashMap<TxnId, Vec<TxnId>>>,
}
