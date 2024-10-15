use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use sqlparser::ast::Expr;
use crate::expressions::functions::function_ext::FunctionExt;
use crate::expressions::functions::utils::is_aggregation_function_name;

/// A bound aggregate call, e.g., `sum(x)`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AggCallExpr {
    // The function name
    pub(crate) func: String,

    /// function arguments
    pub(crate) args: Vec<Box<ExpressionTypeImpl>>,

    /// Is distinct aggregation
    pub(crate) is_distinct: bool,
}

impl AggCallExpr {

    pub(crate) fn new(
        func: String,
        args: Vec<Box<ExpressionTypeImpl>>,
        is_distinct: bool,
    ) -> Self {
        Self {
            func,
            args,
            is_distinct
        }
    }

    pub(crate) fn is_aggregation_function(f: &sqlparser::ast::Function) -> bool {
        is_aggregation_function_name(f.name.to_string()) && f.over.is_none()
    }

}

impl Into<ExpressionTypeImpl> for AggCallExpr {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::AggCall(self)
    }
}

impl Expression for AggCallExpr {
    fn has_aggregation(&self) -> bool {
        true
    }

    fn try_parse_from_expr(expr: &Expr, binder: &Binder) -> ParseASTResult<Self>
    where
        Self: Sized
    {
        match expr {
            Expr::Function(f) => {
                if Self::is_aggregation_function(f) {
                    Ok(Self::new(
                        f.name.to_string(),
                        f.parse_args(binder)?,
                        f.is_distinct()
                    ))
                } else {
                    Err(ParseASTError::IncompatibleType)
                }
            }
            _ => Err(ParseASTError::IncompatibleType)

        }
    }
}
