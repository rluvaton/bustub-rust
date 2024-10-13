use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering;
use std::thread;
use atomic::Atomic;
use parking_lot::Mutex;
use common::config::{AtomicTimestamp, TableOID, Timestamp, TxnId, INVALID_TIMESTAMP, TXN_START_ID};
use rid::RID;
use crate::{IsolationLevel, TransactionState, UndoLink, UndoLog};
use expression::AbstractExpressionRef;

/// Transaction tracks information related to a transaction.
pub struct Transaction {
    /* The below fields should be ONLY changed by txn manager (with the txn manager lock held). */

    /// The state of this transaction
    /// Default: `TransactionState::RUNNING`
    state: Atomic<TransactionState>,

    /// The read timestamp
    /// Default: `0`
    read_ts: AtomicTimestamp,

    /// The commit timestamp
    /// Default: `INVALID_TIMESTAMP`
    commit_ts: AtomicTimestamp,

    /// The latch for this transaction for accessing txn-level undo logs
    latch: Mutex<TransactionData>,

    /* The below fields are set when a txn is created and will NEVER be changed. */

    /// The isolation level of the transaction.
    isolation_level: IsolationLevel,

    /// The thread ID which the txn starts from.
    thread_id: thread::ThreadId,

    /// The ID of this transaction.
    txn_id: TxnId,
}

struct TransactionData {
    /// Store undo logs. Other undo logs / table heap will store (txn_id, index) pairs, and therefore
    /// you should only append to this vector or update things in-place without removing anything.
    undo_logs: Vec<UndoLog>,

    /// stores the RID of write tuples
    write_set: HashMap<TableOID, HashSet<RID>>,

    /// store all scan predicates
    scan_predicates: HashMap<TableOID, Vec<AbstractExpressionRef>>,
}

impl Transaction {
    pub fn new(txn_id: TxnId, isolation_level: Option<IsolationLevel>) -> Self {
        Self {
            state: Atomic::new(TransactionState::Running),
            read_ts: AtomicTimestamp::new(0),
            commit_ts: AtomicTimestamp::new(INVALID_TIMESTAMP),

            latch: Mutex::new(TransactionData {
                undo_logs: vec![],
                write_set: HashMap::new(),
                scan_predicates: HashMap::new(),
            }),

            // Readonly vars
            isolation_level: isolation_level.unwrap_or(IsolationLevel::SnapshotIsolation),
            thread_id: thread::current().id(),
            txn_id,
        }
    }


    /// Return the id of the thread running the transaction
    pub fn get_thread_id(&self) -> thread::ThreadId {
        self.thread_id
    }

    /// Returns the id of this transaction
    pub fn get_transaction_id(&self) -> TxnId { self.txn_id }

    /// Return the id of this transaction, stripping the highest bit. NEVER use/store this value unless for debugging.
    pub fn get_transaction_id_human_readable(&self) -> TxnId { self.txn_id ^ TXN_START_ID }

    /// Return the temporary timestamp of this transaction
    pub fn get_transaction_temp_ts(&self) -> Timestamp {
        // TODO - check why this is txm id and not timestamp
        self.txn_id
    }


    /// Return the isolation level of this transaction
    pub fn get_isolation_level(&self) -> IsolationLevel { self.isolation_level.clone() }

    /// Return the transaction state
    pub fn get_transaction_state(&self) -> TransactionState { self.state.load(Ordering::SeqCst) }
    pub fn set_transaction_state(&self, state: TransactionState)  { self.state.store(state, Ordering::SeqCst) }

    /// Return the read ts
    pub fn get_read_ts(&self) -> Timestamp { self.read_ts.load(Ordering::SeqCst) }

    /// Return the commit ts
    pub fn get_commit_ts(&self) -> Timestamp { self.commit_ts.load(Ordering::SeqCst) }

    /// Modify an existing undo log.
    pub fn modify_undo_log(&self, log_idx: i32, new_log: UndoLog) {
        let mut guard = self.latch.lock();
        guard.undo_logs[log_idx as usize] = new_log
    }

    /// Return the index of the undo log in this transaction
    pub fn append_undo_log(&self, log: UndoLog) -> UndoLink {
        let mut guard = self.latch.lock();
        guard.undo_logs.push(log);

        UndoLink::new(self.txn_id, guard.undo_logs.len() as i32 - 1)
    }

    pub fn append_write_set(&self, t: &TableOID, rid: RID) {
        let mut guard = self.latch.lock();
        if !guard.write_set.contains_key(t) {
            guard.write_set.insert(*t, HashSet::new());
        }
        guard.write_set.get_mut(t).unwrap().insert(rid);
    }

    pub fn get_write_sets(&self) -> HashMap<TableOID, HashSet<RID>> {
        // It's behind a lock, should get a guard instead
        unimplemented!()
        // let guard = self.latch.lock();
        //
        // guard.write_set
    }

    pub fn append_scan_predicate(&self, t: &TableOID, predicate: AbstractExpressionRef) {
        let mut guard = self.latch.lock();
        if !guard.scan_predicates.contains_key(t) {
            guard.scan_predicates.insert(*t, vec![]);
        }
        guard.scan_predicates.get_mut(t).unwrap().push(predicate);
    }


    pub fn get_scan_predicates(&self) -> HashMap<TableOID, Vec<AbstractExpressionRef>> {
        // It's behind a lock, should get a guard instead
        unimplemented!()
        // let guard = self.latch.lock();
        //
        // guard.scan_predicates
    }

    pub fn get_undo_log(&self, log_id: isize) -> UndoLog {
        // TODO - this will create a new undo log,
        // so if want to modify the undo log this should be changed to not return cloned undo log
        // or if not wanting to clone without need
        let guard = self.latch.lock();

        guard.undo_logs[log_id as usize].clone()
    }

    pub fn get_undo_log_num(&self) -> usize {
        let guard = self.latch.lock();
        guard.undo_logs.len()
    }

    /// Use this function in leaderboard benchmarks for online garbage collection. For stop-the-world GC, simply remove
    /// the txn from the txn_map.
    pub fn clear_undo_log(&self) -> usize {
        // TODO - this does not change clear the log or anything
        let guard = self.latch.lock();
        guard.undo_logs.len()
    }

    pub fn set_tainted(&self) {
        let state = self.state.load(Ordering::SeqCst);

        if matches!(state, TransactionState::Running) {
            self.state.store(TransactionState::Tainted, Ordering::SeqCst);
            return
        }

        panic!("transaction not in running state: {:?}", state)
    }
}
