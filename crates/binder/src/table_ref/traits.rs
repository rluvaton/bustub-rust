use crate::expressions::ColumnRef;
use crate::table_ref::table_reference_type::TableReferenceTypeImpl;
use crate::try_from_ast_error::ParseASTResult;
use crate::Binder;
use sqlparser::ast::TableFactor;
use std::fmt::Debug;

/// A bound table reference.
pub trait TableRef: Debug + PartialEq + Into<TableReferenceTypeImpl> {

    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>>;

    fn try_from_ast<'a>(ast: &TableFactor, binder: &'a Binder) -> ParseASTResult<Self>;
}


