use crate::expressions::{Expression, ExpressionTypeImpl};
use std::sync::Arc;
use sqlparser::ast::OrderByExpr;
use crate::Binder;
use crate::try_from_ast_error::ParseASTResult;

/// All types of order-bys in binder.

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum OrderByType {
    Invalid,
    Default,
    Asc,
    Desc
}

impl Default for OrderByType {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OrderBy {
    /// The order by type
    order_type: OrderByType,

    /// The order by expression
    expr: Box<ExpressionTypeImpl>
}

impl OrderBy {
    pub fn new(
        order_type: OrderByType,
        expr: Box<ExpressionTypeImpl>
    ) -> Self {
        Self {
            order_type,
            expr
        }
    }

    pub(crate) fn parse_from_ast(ast: &OrderByExpr, binder: &mut Binder) -> ParseASTResult<Self> {
        let order_by: OrderByType = if let Some(asc) = ast.asc {
            if asc {
                OrderByType::Asc
            } else {
                OrderByType::Desc
            }
        } else { OrderByType::Default };

        Ok(OrderBy::new(
            order_by,
            Box::new(ExpressionTypeImpl::try_parse_from_expr(&ast.expr, binder)?)
        ))
    }
}
