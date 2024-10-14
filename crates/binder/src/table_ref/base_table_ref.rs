use std::sync::Arc;
use common::config::TableOID;
use db_core::catalog::Schema;
use crate::Binder;
use crate::table_ref::table_reference_type::{TableReferenceType, TableReferenceTypeImpl};
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTResult, ParseASTError};

/// A bound table ref type for single table. e.g., `SELECT x FROM y`, where `y` is `BoundBaseTableRef`.
///
#[derive(Debug, PartialEq)]
pub struct BaseTableRef {

    /// The name of the table.
    pub(crate) table: String,

    // The OID of the table
    pub(crate) oid: TableOID,

    // The alias of the table
    pub(crate) alias: Option<String>,

    // The schema of the table
    pub(crate) schema: Arc<Schema>
}

impl BaseTableRef {
    pub fn new(table: String, oid: TableOID, alias: Option<String>, schema: Arc<Schema>) -> Self {
        Self {
            table,
            oid,
            alias,
            schema
        }
    }

    pub fn get_table_name(&self) -> &String {
        &self.alias.as_ref().unwrap_or(&self.table)
    }

    pub fn try_parse(table_name: String, alias: Option<String>, binder: &mut Binder) -> ParseASTResult<Self> {
        if binder.catalog.is_none() {
            return Err(ParseASTError::FailedParsing("Missing catalog in binder".to_string()))
        }

        let catalog = binder.catalog.unwrap();

        let table_info = catalog.get_table_by_name(&table_name);

        if table_info.is_none() {
            return Err(ParseASTError::FailedParsing(format!("Invalid table {}", table_name)))
        }

        let table_info = table_info.unwrap();

        Ok(BaseTableRef::new(table_name, table_info.get_oid(), alias, table_info.get_schema()))
    }
}

impl TableRef for BaseTableRef {
}



impl From<BaseTableRef> for TableReferenceTypeImpl {
    fn from(value: BaseTableRef) -> Self {
        TableReferenceTypeImpl::BaseTable(value)
    }
}
