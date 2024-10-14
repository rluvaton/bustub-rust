use crate::expressions::ColumnRef;
use crate::table_ref::table_reference_type::TableReferenceTypeImpl;
use crate::table_ref::TableRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use common::config::TableOID;
use db_core::catalog::Schema;
use sqlparser::ast::TableFactor;
use std::sync::Arc;

/// A bound table ref type for single table. e.g., `SELECT x FROM y`, where `y` is `BoundBaseTableRef`.
///
#[derive(Clone, Debug, PartialEq)]
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
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        let bound_table_name = self.get_table_name();

        // Firstly, try directly resolve the column name through schema
        let direct_resolved_expr = ColumnRef::resolve_from_schema(col_name, self.schema.clone())?.map(|mut c| c.prepend(bound_table_name.clone()));

        let mut strip_resolved_expr: Option<ColumnRef> = None;

        // Then, try strip the prefix and match again
        if col_name.len() > 1 {
            // Strip alias and resolve again
            if &col_name[0] == bound_table_name {
                let strip_column_name = &col_name[1..];

                strip_resolved_expr = ColumnRef::resolve_from_schema(strip_column_name, self.schema.clone())?.map(|mut c| c.prepend(bound_table_name.clone()));
            }
        }

        if strip_resolved_expr.is_some() && direct_resolved_expr.is_some() {
            return Err(ParseASTError::FailedParsing(format!("{} is ambiguous in table {}", col_name.join("."), self.table)))
        }

        Ok(strip_resolved_expr.or(direct_resolved_expr))
    }

    fn try_from_ast(ast: &TableFactor, binder: &mut Binder) -> ParseASTResult<Self> {
        match ast {
            TableFactor::Table { alias, name, .. } => {
                Self::try_parse(name.to_string(), alias.as_ref().map(|a| a.name.value.clone()), binder)
            }
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}



impl From<BaseTableRef> for TableReferenceTypeImpl {
    fn from(value: BaseTableRef) -> Self {
        TableReferenceTypeImpl::BaseTable(value)
    }
}