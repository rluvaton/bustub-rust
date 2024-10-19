use common::config::{TableOID, TxnId};
use parking_lot::Mutex;
use rid::RID;
use std::collections::HashMap;
use std::sync::Arc;
use std::thread::JoinHandle;
use transaction::{Transaction, TransactionManager};
use crate::{LockMode, LockRequestQueue};

pub struct LockManager {

    #[allow(unused)]
    transaction_manager: Arc<dyn TransactionManager>,

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
    pub fn new(transaction_manager: Arc<dyn TransactionManager>) -> Self {
        Self {
            transaction_manager,
            table_lock_map: Mutex::new(HashMap::new()),
            row_lock_map: Mutex::new(HashMap::new()),
            waits_for: Mutex::new(HashMap::new()),
            cycle_detection_thread: None
        }
    }

    // TODO - add the rest

    /**
     * Acquire a lock on rid in the given lock_mode.
     * If the transaction already holds a lock on the row, upgrade the lock
     * to the specified lock_mode (if possible).
     *
     * This method should abort the transaction and throw a
     * TransactionAbortException under certain circumstances.
     * See [LOCK_NOTE] in header file.
     *
     * @param txn the transaction requesting the lock upgrade
     * @param lock_mode the lock mode for the requested lock
     * @param oid the table_oid_t of the table the row belongs to
     * @param rid the RID of the row to be locked
     * @return true if the upgrade is successful, false otherwise
     */
    #[allow(unused_variables)]
    pub fn lock_row(&self, txn: Arc<Transaction>, lock_mode: LockMode, oid: &TableOID, rid: &RID) -> bool {
        todo!()
    }
}
