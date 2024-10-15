use std::fmt::{Debug, Formatter};
use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;

/// The star in SELECT list, e.g. `SELECT * FROM x`.
#[derive(Clone, PartialEq)]
pub struct StarExpr;

impl StarExpr {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Into<ExpressionTypeImpl> for StarExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Star(self)
    }
}

impl Expression for StarExpr {
    fn has_aggregation(&self) -> bool {
        unreachable!("`HasAggregation` should not have been called on `StarExpr`.");
    }

    fn try_parse_from_expr(expr: &Expr, _binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Wildcard => Ok(StarExpr::new()),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}

impl Debug for StarExpr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("*")
    }
}
