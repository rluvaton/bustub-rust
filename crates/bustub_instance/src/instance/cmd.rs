use common::config::{TxnId, TXN_START_ID};
use transaction::TransactionManager;
use crate::BustubInstance;
use crate::instance::db_output::SystemOutput;

type SystemOutputResult = error_utils::anyhow::Result<SystemOutput>;

impl BustubInstance {
   pub fn cmd_dbg_mvcc(&self, params: Vec<&str>) -> SystemOutputResult {
       if params.len() != 2 {
           return Err(error_utils::anyhow!("please provide a table name"));
       }

       let table = params[1];

       let catalog = self.catalog.lock();
       if let Some(table_info) = catalog.get_table_by_name(&table) {
           let output = SystemOutput::single_cell(format!("please view the result in the BusTub console (or Chrome DevTools console), table={}", table));

           self.txn_manager.debug("\\dbgmvcc".to_string(), Some(table_info), Some(table_info.get_table_heap()));

           Ok(output)
       } else {
           Err(error_utils::anyhow!("table {} not found", table))
       }
   }

    pub fn cmd_display_tables(&self) -> SystemOutputResult {
        let catalog = self.catalog.lock();
        let table_names = catalog.get_table_names();

        Ok(SystemOutput::new(
            vec!["oid".to_string(), "name".to_string(), "cols".to_string()],
            table_names
                .iter()
                .map(|name| {
                    let table_info = catalog.get_table_by_name(name).expect("Table must exists");

                    vec![
                        table_info.get_oid().to_string(),
                        table_info.get_name().clone(),
                        format!("{}", table_info.get_schema())
                    ]
                })
                .collect(),
            false
        ))
    }

    pub fn cmd_display_indices(&self) -> SystemOutputResult {
        let catalog = self.catalog.lock();
        let table_names = catalog.get_table_names();

        Ok(SystemOutput::new(
            vec!["table_name".to_string(), "index_oid".to_string(), "index_name".to_string(), "index_cols".to_string()],
            table_names
                .iter()
                .map(|table_name| {
                    catalog
                        .get_table_indexes_by_name(table_name)
                        .iter()
                        .map(|index_info| (table_name.clone(), index_info.clone()))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .map(|(table_name, index_info)| {

                    vec![
                        table_name,
                        index_info.get_index_oid().to_string(),
                        index_info.get_name().clone(),
                        format!("{}", index_info.get_key_schema())
                    ]
                })
                .collect(),
            false
        ))
    }

    pub fn cmd_display_help() -> SystemOutputResult {
        Ok(SystemOutput::single_cell(r"(Welcome to the BusTub shell!

\dt: show all tables
\di: show all indices
\dbgmvcc <table>: show version chain of a table
\help: show this message again
\txn: show current txn information
\txn <txn_id>: switch to txn
\txn gc: run garbage collection
\txn -1: exit txn mode

BusTub shell currently only supports a small set of Postgres queries. We'll set
up a doc describing the current status later. It will silently ignore some parts
of the query, so it's normal that you'll get a wrong result when executing
unsupported SQL queries. This shell will be able to run `create table` only
after you have completed the buffer pool manager. It will be able to execute SQL
queries after you have implemented necessary query executors. Use `explain` to
see the execution plan of your query.
)".to_string()))
    }

    pub fn cmd_txn(&mut self, params: Vec<&str>) -> SystemOutputResult {
        if !self.managed_txn_mode {
            return Err(error_utils::anyhow!("only supported in managed mode, please use bustub-shell"));
        }

        if params.len() == 1 {
            return match self.current_txn {
                Some(_) => Ok(SystemOutput::single_cell(self.dump_current_txn(""))),
                None => Err(error_utils::anyhow!("no active txn, each statement starts a new txn."))
            };
        }

        if params.len() == 2 {
            let param1 = &params[0];

            if param1 == &"gc".to_string() {
                self.txn_manager.garbage_collection();
                return Ok(SystemOutput::single_cell("GC complete".to_string()));
            }

            let txn_id = param1.parse::<TxnId>().expect("param1 must be a transaction id");

            if txn_id == -1 {
                let dump_txn = self.dump_current_txn("pause current txn ");

                // Remove current transaction
                self.current_txn = None;

                return Ok(SystemOutput::single_cell(dump_txn));
            }

            let transaction = self.txn_manager
                .get_transaction_by_id(txn_id)
                .or_else(|| self.txn_manager.get_transaction_by_id(txn_id + TXN_START_ID));

            return if let Some(transaction) = transaction {
                self.current_txn = Some(transaction);
                Ok(SystemOutput::single_cell(self.dump_current_txn("switch to new txn ")))
            } else {
                Err(error_utils::anyhow!("cannot find txn."))
            }
        }

        Err(error_utils::anyhow!("unsupported txn cmd."))
    }
}
