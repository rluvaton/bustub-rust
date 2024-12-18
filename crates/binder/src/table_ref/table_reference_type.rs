use crate::expressions::ColumnRef;
use crate::table_ref::base_table_ref::BaseTableRef;
use crate::table_ref::cross_product_ref::CrossProductRef;
use crate::table_ref::cte_ref::CTERef;
use crate::table_ref::join_ref::JoinRef;
use crate::table_ref::{ExpressionListRef, SubqueryRef, TableRef};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::{fallback_on_incompatible_2_args, Binder, ExpressionTypeImpl};
use sqlparser::ast::{TableFactor, TableWithJoins};
use std::ops::Deref;


#[derive(Clone, Debug, PartialEq)]
pub enum TableReferenceTypeImpl {
    Invalid,         // < Invalid table reference type.
    BaseTable(BaseTableRef),
    Join(JoinRef),
    ExpressionList(ExpressionListRef), // < Values clause.
    CrossProduct(CrossProductRef),
    SubQuery(SubqueryRef),        // < Subquery.
    CTE(CTERef),
    Empty,            // < Placeholder for empty FROM.
}

impl TableReferenceTypeImpl {
    pub(crate) fn try_to_parse_tables_with_joins<'a>(tables: &[TableWithJoins], binder: &'a Binder<'a>) -> ParseASTResult<TableReferenceTypeImpl> {
        let ctx_guard = binder.new_context();

        match tables.len() {
            0 => Ok(TableReferenceTypeImpl::Empty),
            1 => TableReferenceTypeImpl::parse_from_table_with_join(&tables[0], ctx_guard.deref()),
            _ => CrossProductRef::try_to_parse_tables_with_joins(tables, ctx_guard.deref())
        }
    }

    pub(crate) fn parse_from_table_with_join(ast: &TableWithJoins, binder: &Binder) -> ParseASTResult<Self> {
        if ast.joins.is_empty() {
            Self::try_from_ast(&ast.relation, binder)
        } else {
            JoinRef::parse_from_table_with_join(ast, binder)
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            TableReferenceTypeImpl::Invalid => "Invalid",
            TableReferenceTypeImpl::BaseTable(_) => "BaseTable",
            TableReferenceTypeImpl::Join(_) => "Join",
            TableReferenceTypeImpl::ExpressionList(_) => "ExpressionList",
            TableReferenceTypeImpl::CrossProduct(_) => "CrossProduct",
            TableReferenceTypeImpl::SubQuery(_) => "SubQuery",
            TableReferenceTypeImpl::CTE(_) => "CTE",
            TableReferenceTypeImpl::Empty => "Empty"
        }
    }
}

impl TableRef for TableReferenceTypeImpl {
    fn resolve_column(&self, col_name: &[String], binder: &Binder) -> ParseASTResult<Option<ColumnRef>> {
        match &self {
            TableReferenceTypeImpl::Invalid => panic!("Cant resolve column in invalid scope"),
            TableReferenceTypeImpl::BaseTable(b) => b.resolve_column(col_name, binder),
            TableReferenceTypeImpl::Join(b) => b.resolve_column(col_name, binder),
            TableReferenceTypeImpl::ExpressionList(b) => b.resolve_column(col_name, binder),
            TableReferenceTypeImpl::CrossProduct(b) => b.resolve_column(col_name, binder),
            TableReferenceTypeImpl::SubQuery(b) => b.resolve_column(col_name, binder),
            TableReferenceTypeImpl::CTE(b) => b.resolve_column(col_name, binder),
            TableReferenceTypeImpl::Empty => Err(ParseASTError::FailedParsing(format!("column {} not found", col_name.join("."))))
        }
    }

    fn get_all_columns(&self, binder: &Binder) -> ParseASTResult<Vec<ExpressionTypeImpl>> {
        match &self {
            TableReferenceTypeImpl::BaseTable(t) => t.get_all_columns(binder),
            TableReferenceTypeImpl::Join(t) => t.get_all_columns(binder),
            TableReferenceTypeImpl::CrossProduct(t) => t.get_all_columns(binder),
            TableReferenceTypeImpl::SubQuery(t) => t.get_all_columns(binder),
            TableReferenceTypeImpl::CTE(t) => t.get_all_columns(binder),
            TableReferenceTypeImpl::ExpressionList(t) => t.get_all_columns(binder),
            // TODO - remove the select * from error message
            TableReferenceTypeImpl::Empty => Err(ParseASTError::FailedParsing(format!("select * cannot be used with {:?}", self))),
            TableReferenceTypeImpl::Invalid => panic!("Cant get all columns in invalid scope"),
        }
    }

    fn try_from_ast(ast: &TableFactor, binder: &Binder) -> ParseASTResult<Self> {
        fallback_on_incompatible_2_args!(try_from_ast, ast, binder, {
            BaseTableRef,
            JoinRef,
            ExpressionListRef,
            CrossProductRef,
            SubqueryRef,
            CTERef
        });

        Err(ParseASTError::IncompatibleType)
    }
}
