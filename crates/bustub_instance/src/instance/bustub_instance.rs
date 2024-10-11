use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use buffer_pool_manager::BufferPoolManager;
use checkpoint_manager::CheckpointManager;
use db_core::catalog::Catalog;
use db_core::concurrency::{LockManager, TransactionManager};
use disk_storage::{DefaultDiskManager, DiskManager};
use execution_engine::ExecutionEngine;
use recovery_log_manager::LogManager;

const DEFAULT_BPM_SIZE: usize = 128;
const LRU_K_REPLACER_K: usize = 10;

pub struct BustubInstance {
    disk_manager: Arc<dyn DiskManager>,
    buffer_pool_manager: Arc<BufferPoolManager>,
    lock_manager: Option<Arc<LockManager>>,
    txn_manager: Arc<TransactionManager>,
    log_manager: Option<Arc<LogManager>>,
    checkpoint_manager: Option<Arc<CheckpointManager>>,
    catalog: Arc<Catalog>,
    execution_engine: Arc<ExecutionEngine>,

    // TODO - change this
    catalog_mutex: Mutex<()>,
}

impl BustubInstance {

    /// Create bustub instance from file
    ///
    /// the default bpm size is `DEFAULT_BPM_SIZE`
    pub fn from_file(db_file_path: PathBuf, bpm_size: Option<usize>) -> Self {
        // TODO - add global enable logging

        let disk_manager = Arc::new(DefaultDiskManager::new(db_file_path).expect("disk manager failed to initialize"));

        let mut log_manager: Option<Arc<LogManager>> = None;

        #[cfg(feature = "checkpoint_manager")]
        let log_manager = Some(Arc::new(LogManager::new(disk_manager.clone())));

        // We need more frames for GenerateTestTable to work. Therefore, we use 128 instead of the default
        // buffer pool size specified in `config.h`.

        let bpm = BufferPoolManager::builder()
            .with_pool_size(bpm_size.unwrap_or(DEFAULT_BPM_SIZE))
            .with_arc_disk_manager(disk_manager.clone())
            .with_lru_k_eviction_policy(LRU_K_REPLACER_K)
            .with_log_manager(log_manager.clone()).build_arc();

        let mut lock_manager: Option<Arc<LockManager>> = None;

        #[cfg(feature = "lock_manager")]
        {
            // WE HAVE CIRCULAR DEPS,
            lock_manager = Some(LockManager::new(txn_manager.clone()));
            unimplemented!();

        }

        // let txn_manager = {
        //
        //     #[cfg(feature = "lock_manager")]
        //     {
        //         lock_manager = Some(Arc::new(LockManager::new()));
        //
        //         Arc::new(TransactionManager::new())
        //     }
        //
        //     #[cfg(not(feature="lock_manager"))]
        //     TransactionManager::new()
        //
        //     let mut log_manager: Option<Arc<LogManager>> = None;
        //
        // }


        #[cfg(feature = "checkpoint_manager")]
        let checkpoint_manager = Some(Arc::new(CheckpointManager::new(None, log_manager.clone().unwrap(), bpm.clone())));

        #[cfg(not(feature = "checkpoint_manager"))]
        let mut checkpoint_mgr: Option<Arc<CheckpointManager>> = None;


        let catalog = Arc::new(Catalog::new(bpm.clone(), lock_manager.clone(), log_manager.clone()));
        let txn_manager = Arc::new(TransactionManager::new(catalog.clone()));

        let execution_engine = Arc::new(ExecutionEngine::new(
            bpm.clone(),
            txn_manager.clone(),
            catalog.clone()
        ));

        Self {
            buffer_pool_manager: bpm,
            txn_manager,
            lock_manager,
            log_manager,
            disk_manager: disk_manager,
            checkpoint_manager,
            catalog: catalog.clone(),
            catalog_mutex: Mutex::new(()),
            execution_engine
        }
    }
}
