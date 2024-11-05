use std::sync::Arc;
use buffer_pool_manager::BufferPoolManager;
use db_core::concurrency::TransactionManager;
use recovery_log_manager::LogManager;

pub struct CheckpointManager {
    #[allow(dead_code)]
    transaction_manager: Option<Arc<TransactionManager>>,

    #[allow(dead_code)]
    log_manager: Arc<LogManager>,

    #[allow(dead_code)]
    buffer_pool_manager: Arc<BufferPoolManager>,
}

impl CheckpointManager {
    pub fn new(transaction_manager: Option<Arc<TransactionManager>>,
               log_manager: Arc<LogManager>,
               buffer_pool_manager: Arc<BufferPoolManager>) -> Self {
        Self {
            transaction_manager,
            log_manager,
            buffer_pool_manager
        }
    }
}
