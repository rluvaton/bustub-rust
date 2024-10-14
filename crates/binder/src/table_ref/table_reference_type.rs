use sqlparser::ast::Expr;
use crate::table_ref::{ExpressionListRef, SubqueryRef};
use crate::table_ref::base_table_ref::BaseTableRef;
use crate::table_ref::cross_product_ref::CrossProductRef;
use crate::table_ref::cte_ref::CTERef;
use crate::table_ref::join_ref::JoinRef;

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

#[derive(Debug, PartialEq)]
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
