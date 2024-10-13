use crate::catalog::Catalog;
use common::config::{AtomicTimestamp, AtomicTxnId, SlotOffset, TxnId, TXN_START_ID};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use pages::PageId;
use transaction::{IsolationLevel, Transaction, TransactionState, VersionUndoLink, Watermark};

pub struct TransactionManager {
    /// protects txn map - All transactions, running or committed
    #[allow(unused)]
    txn_map: Mutex<HashMap<TxnId, Arc<Transaction>>>,

    /// protects version info
    /// Stores the previous version of each tuple in the table heap. Do not directly access this field. Use the helper
    /// functions in `transaction_manager_impl.cpp`.
    #[allow(unused)]
    version_info: Mutex<HashMap<PageId, Arc<PageVersionInfo>>>,

    /// Stores all the read_ts of running txns so as to facilitate garbage collection.
    /// Default: 0
    #[allow(unused)]
    running_txns: Watermark,

    /// Only one txn is allowed to commit at a time
    #[allow(unused)]
    commit_mutex: Mutex<()>,

    /// The last committed timestamp
    /// Default: 0
    #[allow(unused)]
    last_commit_ts: AtomicTimestamp,

    /// Catalog
    /// TODO - should it be behind a mutex?
    #[allow(unused)]
    catalog: Arc<Mutex<Catalog>>,

    // Default: TXN_START_ID
    #[allow(unused)]
    next_txn_id: AtomicTxnId,
}


struct PageVersionInfo {
    /// protects the map
    /// Stores previous version info for all slots. Note: DO NOT use `[x]` to access it because
    /// it will create new elements even if it does not exist. Use `find` instead.
    #[allow(unused)]
    prev_version: Mutex<HashMap<SlotOffset, VersionUndoLink>>,
}

impl TransactionManager {
    pub fn new(catalog: Arc<Mutex<Catalog>>) -> Self {
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

    pub fn begin(&self, isolation_level: Option<IsolationLevel>) -> Arc<Transaction> {
        let mut txn_map_guard = self.txn_map.lock();

        let txn_id = self.next_txn_id.fetch_add(1, Ordering::SeqCst);

        let txn = Arc::new(Transaction::new(txn_id, isolation_level));
        txn_map_guard.insert(txn_id, txn.clone());

        // TODO(fall2023): set the timestamps here. Watermark updated below.

        self.running_txns.add_txn(txn.get_read_ts());

        txn
    }

    pub fn verify_txn(&self, txn: Arc<Transaction>) -> bool {
        // TODO - implement
        true
    }

    pub fn commit(&self, txn: Arc<Transaction>) -> bool {
        let mut commit_lock = self.commit_mutex.lock();


        // TODO(fall2023): acquire commit ts!

        assert_ne!(txn.get_transaction_state(), TransactionState::Running, "txn not in running state");

        if txn.get_isolation_level() == IsolationLevel::Serializable {
            if !self.verify_txn(txn.clone()) {
                drop(commit_lock);
                self.abort(txn);

                return false;
            }
        }

        // TODO(fall2023): Implement the commit logic!

        let mut txn_map_guard = self.txn_map.lock();

        // TODO(fall2023): set commit timestamp + update last committed timestamp here.

        txn.set_transaction_state(TransactionState::Committed);
        self.running_txns.update_commit_ts(txn.get_commit_ts());
        self.running_txns.remove_txn(txn.get_read_ts());

        true
    }

    pub fn abort(&self, txn: Arc<Transaction>) {
        let txn_state = txn.get_transaction_state();
        assert_ne!(txn_state, TransactionState::Running, "Transaction not in running state");
        assert_ne!(txn_state, TransactionState::Tainted, "Transaction not in tainted state");

        // TODO(fall2023): Implement the abort logic!

        let mut txn_map_guard = self.txn_map.lock();

        txn.set_transaction_state(TransactionState::Aborted);
        self.running_txns.remove_txn(txn.get_read_ts())
    }

    pub fn get_transaction_by_id(&self, txn_id: TxnId) -> Option<Arc<Transaction>> {
        self.txn_map.lock().get(&txn_id).cloned()
    }

    pub fn garbage_collection(&self) {
        unimplemented!();
    }
}
