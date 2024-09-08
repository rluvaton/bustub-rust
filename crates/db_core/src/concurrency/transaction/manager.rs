use crate::catalog::Catalog;
use crate::concurrency::{Transaction, VersionUndoLink, Watermark};
use common::config::{AtomicTimestamp, AtomicTxnId, PageId, SlotOffset, TxnId, TXN_START_ID};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TransactionManager {
    /// protects txn map - All transactions, running or committed
    txn_map: Mutex<HashMap<TxnId, Arc<Transaction>>>,

    /// protects version info
    /// Stores the previous version of each tuple in the table heap. Do not directly access this field. Use the helper
    /// functions in `transaction_manager_impl.cpp`.
    version_info: Mutex<HashMap<PageId, Arc<PageVersionInfo>>>,

    /// Stores all the read_ts of running txns so as to facilitate garbage collection.
    /// Default: 0
    running_txns: Watermark,

    /// Only one txn is allowed to commit at a time
    commit_mutex: Mutex<()>,

    /// The last committed timestamp
    /// Default: 0
    last_commit_ts: AtomicTimestamp,

    /// Catalog
    catalog: Arc<Catalog>,

    // Default: TXN_START_ID
    next_txn_id: AtomicTxnId,
}


struct PageVersionInfo {
    /// protects the map
    /// Stores previous version info for all slots. Note: DO NOT use `[x]` to access it because
    /// it will create new elements even if it does not exist. Use `find` instead.
    prev_version: Mutex<HashMap<SlotOffset, VersionUndoLink>>,
}

impl TransactionManager {
    pub fn new(catalog: Arc<Catalog>) -> Self {
        Self {
            txn_map: Mutex::new(HashMap::new()),
            version_info: Mutex::new(HashMap::new()),
            running_txns: Watermark::new(0),
            commit_mutex: Mutex::new(()),
            last_commit_ts: AtomicTimestamp::new(0),
            catalog,
            next_txn_id: AtomicTxnId::new(TXN_START_ID),
        }
    }
}
