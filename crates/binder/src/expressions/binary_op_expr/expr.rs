use crate::expressions::{BinaryOp, Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;


/// A bound binary operator, e.g., `a+b`.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOpExpr {
    /// Operator name.
    pub op: BinaryOp,

    /// Left argument of the op.
    pub larg: Box<ExpressionTypeImpl>,

    /// Right argument of the op.
    pub rarg: Box<ExpressionTypeImpl>,
}

impl Into<ExpressionTypeImpl> for BinaryOpExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::BinaryOp(self)
    }
}

impl Expression for BinaryOpExpr {
    fn has_aggregation(&self) -> bool {
        self.larg.has_aggregation() || self.rarg.has_aggregation()
    }


    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::BinaryOp { left, op, right } => Ok(
                BinaryOpExpr {
                    op: op.try_into()?,
                    larg: Box::new(ExpressionTypeImpl::try_parse_from_expr(left, binder)?),
                    rarg: Box::new(ExpressionTypeImpl::try_parse_from_expr(right, binder)?),
                }
            ),
            _ => Err(ParseASTError::IncompatibleType)
        }
    }
}
