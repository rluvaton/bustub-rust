use crate::Binder;
use crate::expressions::ColumnRef;
use crate::table_ref::{ExpressionListRef, SubqueryRef, TableRef};
use crate::table_ref::base_table_ref::BaseTableRef;
use crate::table_ref::cross_product_ref::CrossProductRef;
use crate::table_ref::cte_ref::CTERef;
use crate::table_ref::join_ref::JoinRef;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

#[derive(Debug, PartialEq)]
pub enum TableReferenceType {
    Invalid = 0,         // < Invalid table reference type.
    BaseTable = 1,      // < Base table reference.
    Join = 3,            // < Output of join.
    CrossProduct = 4,   // < Output of cartesian product.
    ExpressionList = 5, // < Values clause.
    SubQuery = 6,        // < Subquery.
    CTE = 7,             // < CTE.
    Empty = 8            // < Placeholder for empty FROM.
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableReferenceTypeImpl {
    Invalid,         // < Invalid table reference type.
    BaseTable(BaseTableRef),
    Join(JoinRef),
    ExpressionList(ExpressionListRef), // < Values clause.
    CrossProduct(CrossProductRef),
    SubQuery(SubqueryRef),        // < Subquery.
    CTE(CTERef),
    Empty            // < Placeholder for empty FROM.
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
}
