use std::sync::Arc;
use parking_lot::Mutex;
use buffer_pool_manager::BufferPoolManager;
use db_core::catalog::Catalog;
use db_core::concurrency::TransactionManager;

pub struct ExecutionEngine {
    bpm: Arc<BufferPoolManager>,
    txn_manager: Arc<TransactionManager>,
    catalog: Arc<Mutex<Catalog>>,
}

impl ExecutionEngine {
    pub fn new(bpm: Arc<BufferPoolManager>,
               txn_manager: Arc<TransactionManager>,
               catalog: Arc<Mutex<Catalog>>) -> Self {
        Self {
            bpm,
            txn_manager,
            catalog
        }
    }
}
