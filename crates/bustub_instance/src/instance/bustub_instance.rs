use std::collections::HashMap;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use parking_lot::Mutex;
use strum::IntoEnumIterator;
use binder::Binder;
use buffer_pool_manager::BufferPoolManager;
use checkpoint_manager::CheckpointManager;
use common::config::{TxnId, TXN_START_ID};
use db_core::catalog::Catalog;
use db_core::concurrency::{LockManager, TransactionManager};
use disk_storage::{DefaultDiskManager, DiskManager, DiskManagerUnlimitedMemory};
use execution_common::CheckOptions;
use execution_engine::ExecutionEngine;
use recovery_log_manager::LogManager;
use crate::mocks::MockTableName;
use transaction::{Transaction};
use crate::result_writer::ResultWriter;

const DEFAULT_BPM_SIZE: usize = 128;
const LRU_K_REPLACER_K: usize = 10;


pub struct BustubInstance {
    pub(super) disk_manager: Arc<dyn DiskManager>,
    pub(super) buffer_pool_manager: Arc<BufferPoolManager>,
    pub(super) lock_manager: Option<Arc<LockManager>>,
    pub(super) txn_manager: Arc<TransactionManager>,
    pub(super) log_manager: Option<Arc<LogManager>>,
    pub(super) checkpoint_manager: Option<Arc<CheckpointManager>>,
    pub(super) execution_engine: Arc<ExecutionEngine>,
    pub(super) catalog: Arc<Mutex<Catalog>>,

    pub(super) session_variables: HashMap<String, String>,
    pub(super) current_txn: Option<Arc<Transaction>>,
    pub(super) managed_txn_mode: bool,
}

impl BustubInstance {
    /// Create bustub instance from file
    ///
    /// the default bpm size is `DEFAULT_BPM_SIZE`
    pub fn from_file(db_file_path: PathBuf, bpm_size: Option<usize>) -> Self {
        Self::create_from_disk_manager(DefaultDiskManager::new(db_file_path).expect("disk manager failed to initialize"), bpm_size)
    }

    /// Create bustub instance in memory
    ///
    /// the default bpm size is `DEFAULT_BPM_SIZE`
    pub fn in_memory(bpm_size: Option<usize>) -> Self {
        Self::create_from_disk_manager(DiskManagerUnlimitedMemory::new(), bpm_size)
    }

    /// Create bustub instance from provided disk manager
    ///
    /// the default bpm size is `DEFAULT_BPM_SIZE`
    fn create_from_disk_manager<DiskManagerImpl: DiskManager>(disk_manager: DiskManagerImpl, bpm_size: Option<usize>) -> Self {
        // TODO - add global enable logging variable should be false

        let disk_manager = Arc::new(disk_manager);

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


        let catalog = Arc::new(Mutex::new(Catalog::new(Some(bpm.clone()), lock_manager.clone(), log_manager.clone())));
        let txn_manager = Arc::new(TransactionManager::new(catalog.clone()));

        let execution_engine = Arc::new(ExecutionEngine::new(
            bpm.clone(),
            txn_manager.clone(),
            catalog.clone(),
        ));

        Self {
            buffer_pool_manager: bpm,
            txn_manager,
            lock_manager,
            log_manager,
            disk_manager,
            checkpoint_manager,
            catalog,
            execution_engine,

            session_variables: HashMap::new(),
            current_txn: None,
            managed_txn_mode: false,
        }
    }

    /// FOR TEST ONLY. Generate test tables in this BusTub instance.
    /// It's used in the shell to predefine some tables, as we don't support
    /// create / drop table and insert for now. Should remove it in the future.
    pub fn generate_mock_table(&self) {
        // The actual content generated by mock scan executors are described in `mock_scan_executor.cpp`.
        let txn = self.txn_manager.begin(None);

        let mut catalog_guard = self.catalog.lock();

        for table_name in MockTableName::iter() {
            let _ = catalog_guard.create_table(txn.clone(), table_name.to_string(), Arc::new(table_name.get_schema()), Some(false));
        }

        drop(catalog_guard);

        self.txn_manager.commit(txn);
    }

    /// Enable managed txn mode on this BusTub instance, allowing statements like `BEGIN`
    pub fn enable_managed_txn(&mut self) {
        self.managed_txn_mode = true;
    }

    /// Get the current transaction.
    pub fn current_managed_txn(&self) -> Option<Arc<Transaction>> {
        self.current_txn.clone()
    }

    pub fn dump_current_txn<ResultWriterImpl: ResultWriter>(&self, writer: &mut ResultWriterImpl, prefix: &'static str) {
        let current_txn = self.current_txn.as_ref().expect("Must have current transaction").clone();

        writer.one_cell(format!("{}txn_id={} txn_real_id={} read_ts={} commit_ts={} status={:?} iso_lvl={:?}",
                    prefix,
                    current_txn.get_transaction_id_human_readable(),
                    current_txn.get_transaction_id(),
                    current_txn.get_read_ts(),
                    current_txn.get_commit_ts(),
                    current_txn.get_transaction_state(),
                    current_txn.get_isolation_level()
            ).as_str());
    }

    pub fn execute_sql<ResultWriterImpl: ResultWriter>(&mut self, sql: &str, writer: &mut ResultWriterImpl, check_options: CheckOptions) -> error_utils::anyhow::Result<bool> {
        let is_local_txn = self.current_txn.is_some();

        let txn = self.current_txn.clone().unwrap_or_else(|| self.txn_manager.begin(None));

        let result = self.execute_sql_txn(sql, writer, txn.clone(), check_options);
        if !is_local_txn {
            let res = self.txn_manager.commit(txn);

            // TODO - change this to return result instead
            assert!(res, "Failed to commit txn");
        }

        return result;
    }

    pub fn execute_sql_txn<ResultWriterImpl: ResultWriter>(&mut self, sql: &str, writer: &mut ResultWriterImpl, txn: Arc<Transaction>, check_options: CheckOptions) -> error_utils::anyhow::Result<bool> {
        if sql.starts_with("\\") {
            // Internal meta-commands, like in `psql`.

            match sql {
                "\\dt" => self.cmd_display_tables(writer),
                "\\di" => self.cmd_display_indices(writer),
                "\\help" => Self::cmd_display_help(writer),
                _ => {
                    if sql.starts_with("\\dbgmvcc") {
                        self.cmd_dbg_mvcc(sql.split("").collect(), writer);
                    } else if sql.starts_with("\\txn") {
                        self.cmd_txn(sql.split("").collect(), writer);
                    } else {
                        return Err(error_utils::anyhow::anyhow!("unsupported internal command: {}", sql))
                    }
                }
            }

            return Ok(true);
        }

        let mut is_successful = true;

        let catalog = self.catalog.lock();

        // TODO - REMOVE THIS!
        let binder = Binder::new(catalog.clone());

        let parsed = binder.parse(sql);

        drop(catalog);


        // let binder = Binder

        todo!();
    }

}

impl Drop for BustubInstance {
    fn drop(&mut self) {
        if let Some(log_manager) = &self.log_manager {
            log_manager.stop_flush_thread();
        }
    }
}
