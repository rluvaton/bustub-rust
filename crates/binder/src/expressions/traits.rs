use crate::expressions::ExpressionTypeImpl;
use crate::try_from_ast_error::ParseASTError;
use crate::Binder;
use std::fmt::Debug;

pub(crate) trait Expression: Debug + PartialEq + Into<ExpressionTypeImpl> {

    fn has_aggregation(&self) -> bool;

    fn has_window_function(&self) -> bool {
        false
    }
}

impl Binder<'_> {
    pub(crate) fn parse_expression(&mut self, node: &sqlparser::ast::Expr) -> Result<ExpressionTypeImpl, ParseASTError> {
        todo!()
    }
}
