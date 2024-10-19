use crate::expressions::{Expression, ExpressionTypeImpl};
use sqlparser::ast::OrderByExpr;
use crate::Binder;
use crate::try_from_ast_error::ParseASTResult;

/// All types of order-bys in binder.

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OrderByType {
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
pub struct OrderBy {
    /// The order by type
    pub order_type: OrderByType,

    /// The order by expression
    pub expr: Box<ExpressionTypeImpl>
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

    pub(crate) fn parse_from_ast(ast: &OrderByExpr, binder: &Binder) -> ParseASTResult<Self> {
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

    pub(crate) fn parse_from_order_by(ast: &sqlparser::ast::OrderBy, binder: &Binder) -> ParseASTResult<Vec<Self>> {
        ast.exprs
            .iter()
            .map(|item| OrderBy::parse_from_ast(item, binder))
            .collect::<ParseASTResult<Vec<OrderBy>>>()
    }
}
