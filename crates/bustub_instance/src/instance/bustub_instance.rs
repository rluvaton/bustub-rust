use crate::instance::ddl::StatementHandler;
use crate::result_writer::ResultWriter;
use crate::rows::Rows;
use binder::{Binder, StatementTypeImpl};
use buffer_pool_manager::BufferPoolManager;
use catalog_schema_mocks::MockTableName;
use checkpoint_manager::CheckpointManager;
use db_core::catalog::Catalog;
use db_core::concurrency::TransactionManager;
use disk_storage::{DefaultDiskManager, DiskManager, DiskManagerUnlimitedMemory};
use error_utils::ToAnyhow;
use execution_common::CheckOptions;
use execution_engine::{ExecutionEngine, ExecutorContext};
use lock_manager::LockManager;
use parking_lot::Mutex;
use planner::{PlanNode, Planner};
use recovery_log_manager::LogManager;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::Arc;
use transaction::{Transaction, TransactionManager as TransactionManagerTrait};
use crate::table_generator::TableGenerator;

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

    // TODO - remove double arc
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

        // We need more frames for generate_test_table to work. Therefore, we use 128 instead of the default
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
        let mut checkpoint_manager: Option<Arc<CheckpointManager>> = None;


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

        for table_name in MockTableName::create_iter() {
            let _ = catalog_guard.create_table(txn.clone(), table_name.to_string(), Arc::new(table_name.get_schema()), Some(false));
        }

        drop(catalog_guard);

        self.txn_manager.commit(txn);
    }

    pub fn generate_test_table(&self) {
        let txn = self.txn_manager.begin(None);
        let exec_ctx = self.make_executor_context(txn.clone(), false);
        let gen = TableGenerator::from(exec_ctx.deref());
        let mut guard = self.catalog.lock();
        gen.generate_test_tables(guard.deref_mut());
        drop(guard);

        self.txn_manager.commit(txn.clone());
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

    pub fn execute_user_input<ResultWriterImpl: ResultWriter>(&mut self, sql_or_command: &str, writer: &mut ResultWriterImpl, check_options: CheckOptions) -> error_utils::anyhow::Result<()> {
        self.wrap_with_txn(|this, txn|
            this.execute_sql_txn(sql_or_command, writer, txn.clone(), check_options)
        )
    }

    fn wrap_with_txn<R, F: FnOnce(&mut Self, Arc<Transaction>) -> R>(&mut self, f: F) -> R {
        let is_local_txn = self.current_txn.is_some();

        let txn = self.current_txn.clone().unwrap_or_else(|| self.txn_manager.begin(None));

        let result = f(self, txn.clone());

        if !is_local_txn {
            let res = self.txn_manager.commit(txn);

            // TODO - change this to return result instead
            assert!(res, "Failed to commit txn");
        }

        result
    }

    pub fn execute_sql_txn<ResultWriterImpl: ResultWriter>(&mut self, sql: &str, writer: &mut ResultWriterImpl, txn: Arc<Transaction>, _check_options: CheckOptions) -> error_utils::anyhow::Result<()> {
        if sql.starts_with("\\") {
            return self.execute_shell_commands(sql, writer);
        }

        let parsed = self.parse_sql(sql)?;

        for stmt in &parsed {

            match &stmt {
                StatementTypeImpl::Invalid => break,
                StatementTypeImpl::Select(_) => {}
                StatementTypeImpl::Insert(_) => {}
                StatementTypeImpl::Create(stmt) => {
                    self.create_table(txn.clone(), stmt, writer);
                    continue;
                }
                StatementTypeImpl::Delete(_) => {}
            }

            let rows = self.execute_data_stmt(stmt, txn.clone())?;

            rows.write_results(writer);
        }
        
        Ok(())
    }

    /// Execute Single SELECT statement and return the results
    ///
    /// This is useful for testings
    pub fn execute_single_select_sql(&mut self, sql: &str, _check_options: CheckOptions) -> error_utils::anyhow::Result<Rows> {
        self.wrap_with_txn(|this, txn| {
            let parsed = this.parse_sql(sql)?;

            assert_eq!(parsed.len(), 1, "Must have single statement");

            let stmt = &parsed[0];
            assert!(matches!(stmt, StatementTypeImpl::Select(_)), "Statement must be a select statement, instead got {:#?}", stmt);

            this.execute_data_stmt(stmt, txn)
        })
    }

    /// Execute Single INSERT statement and return the results
    ///
    /// This is useful for testings
    pub fn execute_single_insert_sql(&mut self, sql: &str, _check_options: CheckOptions) -> error_utils::anyhow::Result<Rows> {
        self.wrap_with_txn(|this, txn| {
            let parsed = this.parse_sql(sql)?;

            assert_eq!(parsed.len(), 1, "Must have single statement");

            let stmt = &parsed[0];
            assert!(matches!(stmt, StatementTypeImpl::Insert(_)), "Statement must be a insert statement, instead got {:#?}", stmt);

            this.execute_data_stmt(stmt, txn)
        })
    }

    /// Execute Single DELETE statement and return the results
    ///
    /// This is useful for testings
    pub fn execute_single_delete_sql(&mut self, sql: &str, _check_options: CheckOptions) -> error_utils::anyhow::Result<Rows> {
        self.wrap_with_txn(|this, txn| {
            let parsed = this.parse_sql(sql)?;

            assert_eq!(parsed.len(), 1, "Must have single statement");

            let stmt = &parsed[0];
            assert!(matches!(stmt, StatementTypeImpl::Delete(_)), "Statement must be a delete statement, instead got {:#?}", stmt);

            this.execute_data_stmt(stmt, txn)
        })
    }

    /// Execute SELECT/INSERT/DELETE/UPDATE statements
    fn execute_data_stmt(&mut self, stmt: &StatementTypeImpl, txn: Arc<Transaction>) -> error_utils::anyhow::Result<Rows> {
        let mut is_delete = false;

        // Assert SELECT/INSERT/DELETE/UPDATE statements
        match stmt {
            StatementTypeImpl::Select(_) | StatementTypeImpl::Insert(_) => {}
            StatementTypeImpl::Delete(_) => {
                is_delete = true;
            }
            _ => unreachable!()
        }
        let catalog = self.catalog.lock();

        // Plan the query
        let plan = Planner::new(catalog.deref()).plan(stmt);

        // Optimize the query
        // TODO - add back

        drop(catalog);

        // Execute the query.
        // TODO - add executor
        let exec_ctx = self.make_executor_context(txn.clone(), is_delete);
        // TODO - add check options
        let result = self.execution_engine.execute(plan.clone(), txn.clone(), exec_ctx)?;

        // Return the result set as a vector of string.
        let schema = plan.get_output_schema();

        Ok(Rows::new(result, schema))
    }

    fn execute_shell_commands<ResultWriterImpl: ResultWriter>(&mut self, cmd: &str, writer: &mut ResultWriterImpl) -> error_utils::anyhow::Result<()> {
        // Internal meta-commands, like in `psql`.
        assert!(cmd.starts_with("\\"), "command must start with \\");

        match cmd {
            "\\dt" => self.cmd_display_tables(writer),
            "\\di" => self.cmd_display_indices(writer),
            "\\help" => Self::cmd_display_help(writer),
            _ => {
                if cmd.starts_with("\\dbgmvcc") {
                    self.cmd_dbg_mvcc(cmd.split("").collect(), writer);
                } else if cmd.starts_with("\\txn") {
                    self.cmd_txn(cmd.split("").collect(), writer);
                } else {
                    return Err(error_utils::anyhow::anyhow!("unsupported internal command: {}", cmd))
                }
            }
        }

        Ok(())
    }

    fn parse_sql(&mut self, sql: &str) -> error_utils::anyhow::Result<Vec<StatementTypeImpl>> {
        assert_eq!(sql.starts_with("\\"), false, "SQL query must not start with meta command prefix \\");

        let catalog = self.catalog.lock();

        let binder = Binder::new(catalog.deref());

        binder.parse(sql).map_err(|err| err.to_anyhow())
    }

    fn make_executor_context(&self, txn: Arc<Transaction>, is_modify: bool) -> Arc<ExecutorContext> {
        Arc::new(
            ExecutorContext::new(
                txn,
                self.catalog.clone(),
                self.buffer_pool_manager.clone(),
                self.txn_manager.clone(),
                self.lock_manager.clone(),
                is_modify,
            )
        )
    }
    
    pub fn verify_integrity(&self) {
        let txn = self.current_txn.clone().unwrap_or_else(|| self.txn_manager.begin(None));
        {
            let catalog = self.catalog.lock();
            
            catalog.verify_integrity(txn.deref());
        }
    } 
}

impl Drop for BustubInstance {
    fn drop(&mut self) {
        if let Some(log_manager) = &self.log_manager {
            log_manager.stop_flush_thread();
        }
    }
}
