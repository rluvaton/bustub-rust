use crate::concurrency::{LockRequestQueue, TransactionManager};
use common::config::{TableOID, TxnId};
use parking_lot::Mutex;
use rid::RID;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct LockManager {

    #[allow(unused)]
    transaction_manager: Arc<TransactionManager>,

    /// Structure that holds lock requests for a given table oid
    #[allow(unused)]
    table_lock_map: Mutex<HashMap<TableOID, Arc<LockRequestQueue>>>,

    /// Structure that holds lock requests for a given RID
    #[allow(unused)]
    row_lock_map: Mutex<HashMap<RID, Arc<LockRequestQueue>>>,

    // #[allow(unused)]
    // enable_cycle_detection: AtomicBool,

    #[allow(unused)]
    cycle_detection_thread: Option<JoinHandle<()>>,

    /// Waits-for graph representation.
    #[allow(unused)]
    waits_for: Mutex<HashMap<TxnId, Vec<TxnId>>>,
}

impl LockManager {
    pub fn new(transaction_manager: Arc<TransactionManager>) -> Self {
        Self {
            transaction_manager,
            table_lock_map: Mutex::new(HashMap::new()),
            row_lock_map: Mutex::new(HashMap::new()),
            waits_for: Mutex::new(HashMap::new()),
            cycle_detection_thread: None
        }
    }
}
