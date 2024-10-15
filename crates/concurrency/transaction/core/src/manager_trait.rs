use crate::{IsolationLevel, Transaction, UndoLink, UndoLog, VersionUndoLink};
use common::config::Timestamp;
use rid::RID;
use std::sync::Arc;

pub trait CheckUndoLink {
    fn check_undo_link(&self, undo_link: Option<UndoLink>) -> bool;
}

pub trait CheckVersionUndoLink {
    fn check_version_undo_link(&self, version_undo_link: Option<VersionUndoLink>) -> bool;
}

pub trait TransactionManager {

    /// Get the lowest read timestamp in the system
    fn get_watermark(&self) -> Timestamp;

    ///
    /// Begins a new transaction.
    ///     @param isolation_level an optional isolation level of the transaction.
    ///     @return an initialized transaction
    ///
    ///
    /// # Arguments
    ///
    /// * `isolation_level`: isolation_level an optional isolation level of the transaction. Default is `SnapshotIsolation`
    ///
    /// returns: Arc<Transaction, Global> an initialized transaction
    fn begin(&self, isolation_level: Option<IsolationLevel>) -> Arc<Transaction>;

    /// Commits a transaction.
    ///
    /// # Arguments
    ///
    /// * `txn`: txn the transaction to commit, the txn will be managed by the txn manager so no need to delete it by yourself
    ///
    /// returns: bool
    fn commit(&self, txn: Arc<Transaction>) -> bool;

    /// Aborts a transaction
    ///
    /// # Arguments
    ///
    /// * `txn`: txn the transaction to abort, the txn will be managed by the txn manager so no need to delete it by yourself
    ///
    fn abort(&self, txn: Arc<Transaction>);


    /// Stop-the-world garbage collection. Will be called only when all transactions are not accessing the table heap.
    fn garbage_collection(&self);


    /// Use this function before task 4.2. Update an undo link that links table heap tuple to the first undo log.
    /// Before updating, `check` function will be called to ensure validity.
    ///
    /// # Arguments
    ///
    /// * `rid`:
    /// * `prev_link`:
    /// * `check`: function to ensure validity.
    ///
    /// returns: bool
    fn update_undo_link(&mut self, rid: RID, prev_link: Option<UndoLink>, check: Option<&dyn CheckUndoLink>) -> bool;

    /// Use this function after task 4.2. Update an undo link that links table heap tuple to the first undo log.
    /// Before updating, `check` function will be called to ensure validity.
    ///
    /// # Arguments
    ///
    /// * `rid`:
    /// * `prev_version`:
    /// * `check`:  function to ensure validity.
    ///
    /// returns: bool
    ///
    fn update_version_link(&mut self, rid: RID, prev_version: Option<VersionUndoLink>, check: Option<&dyn CheckVersionUndoLink>) -> bool;

    /// Get the first undo log of a table heap tuple.
    ///
    /// Use this before task 4.2
    ///
    /// # Arguments
    ///
    /// * `rid`:
    ///
    /// returns: Option<UndoLink>
    ///
    fn get_undo_link(&self, rid: RID) -> Option<UndoLink>;


    /// Get the first undo log of a table heap tuple.
    ///
    /// Use this after task 4.2
    ///
    /// # Arguments
    ///
    /// * `rid`:
    ///
    /// returns: Option<VersionUndoLink>
    ///
    fn get_version_link(&self, rid: RID) -> Option<VersionUndoLink>;

    /// Access the transaction undo log buffer and get the undo log. Return nullopt if the txn does not exist.
    /// Will still throw an exception if the index is out of range.
    ///
    /// # Arguments
    ///
    /// * `link`:
    ///
    /// returns: Option<UndoLog>
    ///
    fn get_undo_log(&self, link: UndoLink) -> Option<UndoLog>;

    /// Access the transaction undo log buffer and get the undo log. Except when accessing the current txn buffer,
    /// you should always call this function to get the undo log instead of manually retrieve the txn shared_ptr and access
    /// the buffer.
    ///
    /// # Arguments
    ///
    /// * `link`:
    ///
    /// returns: UndoLog
    ///
    /// # Safety
    /// Call this only when you are sure there is a transaction exists
    ///
    /// Prefer to use `get_undo_log`
    ///
    unsafe fn get_undo_log_unchecked(&self, link: UndoLink) -> UndoLog;
}
