use crate::expressions::{Expression, ExpressionTypeImpl, UnaryOp};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;


/// A bound unary operator, e.g., `not a`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UnaryOpExpr {
    /// Operator name.
    pub(crate) op: UnaryOp,

    /// argument for the operator
    pub(crate) arg: Box<ExpressionTypeImpl>,
}

impl UnaryOpExpr {

    pub(crate) fn new(
        op: UnaryOp,
        arg: Box<ExpressionTypeImpl>,
    ) -> Self {
        Self {
            op,
            arg,
        }
    }
}

impl Into<ExpressionTypeImpl> for UnaryOpExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::UnaryOp(self)
    }
}

impl Expression for UnaryOpExpr {
    fn has_aggregation(&self) -> bool {
        self.arg.has_aggregation()
    }

    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::UnaryOp { op, expr } => Ok(
                UnaryOpExpr {
                    op: op.try_into()?,
                    arg: Box::new(ExpressionTypeImpl::try_parse_from_expr(expr, binder)?),
                }
            ),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
