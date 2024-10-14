use std::sync::Arc;
use sqlparser::ast::Expr;
use crate::Binder;
use crate::expressions::{ColumnRef, Expression, ExpressionType, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};

/// The alias in SELECT list, e.g. `SELECT count(x) AS y`, the `y` is an alias.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Alias {

    /// The alias
    pub(crate) alias: String,

    /// The actual expression
    pub(crate) child: Box<ExpressionTypeImpl>
}

impl Alias {

    pub(crate) fn new(alias: String, child: Box<ExpressionTypeImpl>) -> Self {
        Self {
            alias,
            child
        }
    }
}

impl Into<ExpressionTypeImpl> for Alias {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Alias(self)
    }
}

impl Expression for Alias {
    fn has_aggregation(&self) -> bool {
        self.child.has_aggregation()
    }

    fn has_window_function(&self) -> bool {
        self.child.has_window_function()
    }

    fn try_parse_from_expr(expr: &Expr, binder: &mut Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Named { name, expr } => Ok(Alias::new(name.to_string(), Box::new(ExpressionTypeImpl::try_parse_from_expr(expr, binder)?))),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
