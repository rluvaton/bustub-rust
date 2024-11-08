use std::sync::Arc;
use buffer_pool_manager::errors::BufferPoolError;
use catalog_schema::Schema;
use common::config::TableOID;
use table::TableHeap;
use transaction::Transaction;

/// The TableInfo class maintains metadata about a table.
pub struct TableInfo {

    /// The table schema
    schema: Arc<Schema>,

    /// The table name
    name: String,

    /// An owning pointer to the table heap
    table: Arc<TableHeap>,

    /// The table OID
    oid: TableOID,
}

impl TableInfo {
    pub fn new(schema: Arc<Schema>, name: String, table: Arc<TableHeap>, oid: TableOID) -> Self {
        Self {
            schema,
            name,
            table,
            oid
        }
    }

    pub fn get_table_heap(&self) -> Arc<TableHeap> {
        self.table.clone()
    }

    pub fn get_oid(&self) -> TableOID {
        self.oid
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_schema(&self) -> Arc<Schema> {
        // For debugging in bustub instance
        self.schema.clone()
    }

    pub fn delete_completely(self, txn: &Transaction) -> Result<(), BufferPoolError> {
        self.table.delete_completely(txn)
    }
}
