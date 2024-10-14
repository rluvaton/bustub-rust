use crate::expressions::{AggCallExpr, Expression, ExpressionTypeImpl, WindowExpr};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;
use crate::expressions::functions::function_ext::FunctionExt;

/// A bound binary operator, e.g., `a+b`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FuncCallExpr {
    // The function name
    pub(crate) func: String,

    /// function arguments
    pub(crate) args: Vec<Box<ExpressionTypeImpl>>,
}

impl FuncCallExpr {

    pub(crate) fn new(
        func: String,
        args: Vec<Box<ExpressionTypeImpl>>,
    ) -> Self {
        Self {
            func,
            args
        }
    }
}

impl Into<ExpressionTypeImpl> for FuncCallExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::FuncCall(self)
    }
}

impl Expression for FuncCallExpr {
    fn has_aggregation(&self) -> bool {
        false
    }


    fn try_parse_from_expr(expr: &Expr, binder: &mut Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Function(f) => {
                if !AggCallExpr::is_aggregation_function(f) && !WindowExpr::is_window_function(f) {
                    Ok(Self::new(
                        f.name.to_string(),
                        f.parse_args(binder)?,
                    ))
                } else {
                    Err(ParseASTError::IncompatibleType)
                }
            }
            _ => Err(ParseASTError::IncompatibleType)

        }
    }
}
