use std::sync::Arc;
use common::config::TableOID;
use crate::catalog::Schema;
use crate::storage::TableHeap;

/// The TableInfo class maintains metadata about a table.
pub struct TableInfo {

    /// The table schema
    #[allow(unused)]
    schema: Schema,

    /// The table name
    #[allow(unused)]
    name: String,

    /// An owning pointer to the table heap
    #[allow(unused)]
    table: Arc<TableHeap>,

    /// The table OID
    #[allow(unused)]
    oid: TableOID,
}
