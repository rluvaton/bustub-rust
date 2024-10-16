use crate::executors::{Executor, ExecutorRef};
use buffer_pool_manager::BufferPoolManager;
use db_core::catalog::Catalog;
use db_core::concurrency::TransactionManager;
use execution_common::CheckOptions;
use lock_manager::LockManager;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use parking_lot::Mutex;
use recovery_log_manager::LogManager;
use transaction::Transaction;

/// ExecutorContext stores all the context necessary to run an executor.
pub struct ExecutorContext {

    /// The transaction context associated with this executor context
    transaction: Arc<Transaction>,

    /// The database catalog associated with this executor context
    catalog: Arc<Mutex<Catalog>>,

    /// The buffer pool manager associated with this executor context
    bpm: Arc<BufferPoolManager>,

    /// The transaction manager associated with this executor context
    txn_mgr: Arc<TransactionManager>,

    /// The lock manager associated with this executor context
    lock_mgr: Option<Arc<LockManager>>,

    /// The set of NLJ check executors associated with this executor context
    nested_loop_join_check_exec_set: VecDeque<(Box<dyn Executor>, Box<dyn Executor>)>,

    /// The set of check options associated with this executor context
    /// TODO - remove ARC
    check_options: Arc<CheckOptions>,

    is_delete: bool,
}

impl Debug for ExecutorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Executor Context")
            // TODO - add fields
            .finish()
    }
}


impl ExecutorContext {

    /**
     * Creates an ExecutorContext for the transaction that is executing the query.
     * @param transaction The transaction executing the query
     * @param catalog The catalog that the executor uses
     * @param bpm The buffer pool manager that the executor uses
     * @param txn_mgr The transaction manager that the executor uses
     * @param lock_mgr The lock manager that the executor uses
     */
    pub fn new(transaction: Arc<Transaction>, catalog: Arc<Mutex<Catalog>>, bpm: Arc<BufferPoolManager>, txn_mgr: Arc<TransactionManager>,
                      lock_mgr: Option<Arc<LockManager>>, is_delete: bool) -> Self {
        Self {
            transaction,
            catalog,
            bpm,
            txn_mgr,
            lock_mgr,
            nested_loop_join_check_exec_set: VecDeque::new(),
            check_options: Arc::new(CheckOptions::default()),
            is_delete,
        }
    }

    /** @return the running transaction */
    pub(crate) fn get_transaction(&self) -> &Arc<Transaction> {
        &self.transaction
    }


    /** @return the catalog */
    pub(crate) fn get_catalog(&self) -> &Arc<Mutex<Catalog>> { &self.catalog }

    /** @return the buffer pool manager */
    pub(crate) fn get_buffer_pool_manager(&self) -> &Arc<BufferPoolManager> { &self.bpm }

    /** @return the log manager - don't worry about it for now */
    pub(crate) fn get_log_manager(&self) -> &Option<Arc<LogManager>> { &None }

    /** @return the lock manager */
    pub(crate) fn get_lock_manager(&self) -> &Option<Arc<LockManager>> { &self.lock_mgr }

    /** @return the transaction manager */
    pub(crate) fn get_transaction_manager(&self) -> &Arc<TransactionManager> { &self.txn_mgr }


    /** @return the set of nlj check executors */
    pub(crate) fn get_nested_loop_join_check_exec_set(&self) -> &VecDeque<(Box<dyn Executor>, Box<dyn Executor>)> {
        &self.nested_loop_join_check_exec_set
    }

    /** @return the check options */
    pub(crate) fn get_check_options(&self) -> &Arc<CheckOptions> { &self.check_options }


    pub(crate) fn add_check_executor(&mut self, left_exec: Box<dyn Executor>, right_exec: Box<dyn Executor>) {
        self.nested_loop_join_check_exec_set.push_back((left_exec, right_exec));
    }

    pub(crate) fn init_check_options(&mut self, check_options: Arc<CheckOptions>) {
        self.check_options = check_options
    }

    /** As of Fall 2023, this function should not be used. */
    pub(crate) fn is_delete(&self) -> bool {
        self.is_delete
    }
}
