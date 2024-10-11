use std::sync::Arc;
use buffer_pool_manager::BufferPoolManager;
use db_core::concurrency::TransactionManager;
use recovery_log_manager::LogManager;

pub struct CheckpointManager {
    transaction_manager: Arc<TransactionManager>,
    log_manager: Arc<LogManager>,
    buffer_pool_manager: Arc<BufferPoolManager>,
}

impl CheckpointManager {
    pub fn new(transaction_manager: Arc<TransactionManager>,
               log_manager: Arc<LogManager>,
               buffer_pool_manager: Arc<BufferPoolManager>) -> Self {
        Self {
            transaction_manager,
            log_manager,
            buffer_pool_manager
        }
    }
}
