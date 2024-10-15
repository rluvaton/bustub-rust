use crate::instance::ddl::StatementHandler;
use crate::result_writer::ResultWriter;
use crate::BustubInstance;
use binder::CreateStatement;
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::DBTypeId;
use db_core::catalog::{IndexInfo, IndexType};
use index::TWO_INTEGER_SIZE;
use transaction::Transaction;

impl StatementHandler for BustubInstance {
    fn create_table<ResultWriterImpl: ResultWriter>(&self, txn: Arc<Transaction>, stmt: &CreateStatement, writer: &mut ResultWriterImpl) {
        let mut catalog_guard = self.catalog.lock();

        let info = catalog_guard.create_table(
            txn.clone(),
            stmt.table.clone(),
            Arc::new(Schema::new(stmt.columns.clone())),
            None,
        ).expect("Should create table");

        let mut index_info: Option<Arc<IndexInfo>> = None;


        if !stmt.primary_key.is_empty() {
            let col_ids: Vec<u32> = stmt.primary_key
                .iter()
                .map(|col| info.get_schema().get_col_idx(col.to_string()) as u32)
                .collect();

            let has_unsupported_index = col_ids.iter().any(|&col_idx| info.get_schema().get_column(col_idx as usize).get_type() != DBTypeId::INT);

            if has_unsupported_index {
                unimplemented!("only support creating index on integer column");
            }

            let key_schema = Schema::copy_schema(&info.get_schema(), &col_ids);

            // TODO(spring2023): If you want to support composite index key for leaderboard optimization, remove this assertion
            // and create index with different key type that can hold multiple keys based on number of index columns.
            //
            // You can also create clustered index that directly stores value inside the index by modifying the value type.

            if col_ids.is_empty() || col_ids.len() > 2 {
                unimplemented!("only support creating index with exactly one or two columns");
            }

            index_info = catalog_guard.create_index(
                Some(txn),
                (stmt.table.clone() + "_pk").as_str(),
                stmt.table.as_str(),
                info.get_schema(),
                Arc::new(key_schema),
                col_ids.as_slice(),
                TWO_INTEGER_SIZE,
                true,
                IndexType::HashTableIndex,
            );
        }

        drop(catalog_guard);

        match index_info {
            None => writer.one_cell(format!("Table created with id = {}", info.get_oid()).as_str()),
            Some(index) => writer.one_cell(format!("Table created with id = {}, Primary key index created with id = {}", info.get_oid(), index.get_index_oid()).as_str()),
        }
    }
}
