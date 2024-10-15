use crate::expressions::ExpressionTypeImpl;
use crate::try_from_ast_error::{ParseASTError, ParseASTResult};
use crate::Binder;
use std::fmt::Debug;

pub(crate) trait Expression: Debug + PartialEq + Into<ExpressionTypeImpl> {

    fn has_aggregation(&self) -> bool;

    fn has_window_function(&self) -> bool {
        false
    }

    fn try_parse_from_expr(expr: &sqlparser::ast::Expr, binder: &Binder) -> ParseASTResult<Self> where Self: Sized;
}

