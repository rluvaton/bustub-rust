use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;

/// The alias in SELECT list, e.g. `SELECT count(x) AS y`, the `y` is an alias.
#[derive(Clone, Debug, PartialEq)]
pub struct AliasExpr {

    /// The alias
    pub alias: String,

    /// The actual expression
    pub child: Box<ExpressionTypeImpl>
}

impl AliasExpr {

    pub(crate) fn new(alias: String, child: Box<ExpressionTypeImpl>) -> Self {
        Self {
            alias,
            child
        }
    }
}

impl Into<ExpressionTypeImpl> for AliasExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Alias(self)
    }
}

impl Expression for AliasExpr {
    fn has_aggregation(&self) -> bool {
        self.child.has_aggregation()
    }

    fn has_window_function(&self) -> bool {
        self.child.has_window_function()
    }

    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Named { name, expr } => Ok(AliasExpr::new(name.to_string(), Box::new(ExpressionTypeImpl::try_parse_from_expr(expr, binder)?))),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
