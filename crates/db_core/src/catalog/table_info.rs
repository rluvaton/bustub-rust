use std::sync::Arc;
use common::config::TableOID;
use crate::catalog::Schema;
use crate::storage::TableHeap;

/// The TableInfo class maintains metadata about a table.
pub struct TableInfo {

    /// The table schema
    schema: Schema,

    /// The table name
    name: String,

    /// An owning pointer to the table heap
    table: Arc<TableHeap>,

    /// The table OID
    oid: TableOID,
}
