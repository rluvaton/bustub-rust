use crate::table_generator::column_insert_meta::ColumnInsertMeta;

/// Metadata about a table. Specifically, the schema and number of
/// rows in the table.
pub(super) struct TableInsertMeta {
    /// Name of the table
    pub(super) name: String,

    /// Number of rows
    pub(super) num_rows: usize,

    /// Columns
    pub(super) col_meta: Vec<ColumnInsertMeta>,
}


