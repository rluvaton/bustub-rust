use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use parking_lot::Mutex;
use common::config::{TableOID, TxnId};
use rid::RID;
use crate::concurrency::{LockRequestQueue, TransactionManager};

pub struct LockManager {

    #[allow(unused)]
    transaction_manager: Arc<TransactionManager>,

    /// Structure that holds lock requests for a given table oid
    #[allow(unused)]
    table_lock_map: Mutex<HashMap<TableOID, Arc<LockRequestQueue>>>,

    /// Structure that holds lock requests for a given RID
    #[allow(unused)]
    row_lock_map: Mutex<HashMap<RID, Arc<LockRequestQueue>>>,

    #[allow(unused)]
    enable_cycle_detection: AtomicBool,

    #[allow(unused)]
    cycle_detection_thread: JoinHandle<()>,

    /// Waits-for graph representation.
    #[allow(unused)]
    waits_for: Mutex<HashMap<TxnId, Vec<TxnId>>>,
}
