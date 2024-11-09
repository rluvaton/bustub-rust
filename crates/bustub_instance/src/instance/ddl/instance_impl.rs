use std::ops::Deref;
use crate::instance::ddl::StatementHandler;
use crate::BustubInstance;
use binder::{CreateStatement, DropTableStatement};
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::DBTypeId;
use db_core::catalog::{IndexInfo, IndexType};
use index::TWO_INTEGER_SIZE;
use transaction::Transaction;
use crate::instance::db_output::StatementOutput;

impl StatementHandler for BustubInstance {
    fn create_table(&self, txn: Arc<Transaction>, stmt: &CreateStatement) -> error_utils::anyhow::Result<StatementOutput> {
        let mut catalog_guard = self.catalog.lock();

        let info = catalog_guard.create_table(
            txn.clone(),
            stmt.table.clone(),
            Arc::new(Schema::new(stmt.columns.clone())),
            None,
        );

        if info.is_none() {
            return Err(error_utils::anyhow!("Table already exists"));
        }

        let info = info.unwrap();

        let table_oid = info.get_oid();

        let mut index_info: Option<&IndexInfo> = None;

        if !stmt.primary_key.is_empty() {
            let col_ids: Vec<u32> = stmt.primary_key
                .iter()
                .map(|col| info.get_schema().get_col_idx(col) as u32)
                .collect();

            let has_unsupported_index = col_ids.iter().any(|&col_idx| info.get_schema().get_column(col_idx as usize).get_type() != DBTypeId::INT);

            if has_unsupported_index {
                return Err(error_utils::anyhow!("Unimplemented: only support creating index on integer column"));
            }

            let key_schema = Schema::copy_schema(&info.get_schema(), &col_ids);

            // TODO(spring2023): If you want to support composite index key for leaderboard optimization, remove this assertion
            // and create index with different key type that can hold multiple keys based on number of index columns.
            //
            // You can also create clustered index that directly stores value inside the index by modifying the value type.

            if col_ids.is_empty() || col_ids.len() > 2 {
                return Err(error_utils::anyhow!("only support creating index with exactly one or two columns"));
            }

            let schema = info.get_schema();

            index_info = Some(catalog_guard.create_index(
                txn,
                (stmt.table.clone() + "_pk").as_str(),
                stmt.table.as_str(),
                schema,
                Arc::new(key_schema),
                col_ids.as_slice(),
                TWO_INTEGER_SIZE,
                true,
                IndexType::HashTableIndex,
            )?);
        }

        Ok(match index_info {
            None => StatementOutput::new_with_info(1, format!("Table created with id = {}", table_oid)),
            Some(index) => StatementOutput::new_with_info(1, format!("Table created with id = {}, Primary key index created with id = {}", table_oid, index.get_index_oid())),
        })
    }

    fn drop_table(&self, txn: Arc<Transaction>, stmt: &DropTableStatement) -> error_utils::anyhow::Result<StatementOutput> {
        let mut catalog_guard = self.catalog.lock();

        let founded = catalog_guard.drop_table(txn.deref(), stmt.get_table_name().to_string())?;

        match (stmt.get_if_exists(), founded) {
            (false, false) => Err(error_utils::anyhow!("Cannot delete missing table {}, consider using if exists", stmt.get_table_name())),
            (true, false) => Ok(StatementOutput::new_with_info(0, "0 tables deleted".to_string())),
            (_, true) => Ok(StatementOutput::new_with_info(1, format!("Table and all related indexes deleted = {}", stmt.get_table_name())))
        }
    }
}
