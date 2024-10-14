use std::fmt::Debug;
use std::sync::Arc;
use crate::Binder;
use crate::expressions::{ColumnRef, ExpressionType, ExpressionTypeImpl};
use crate::try_from_ast_error::TryFromASTError;

pub(crate) trait Expression: Debug + PartialEq {
    const TYPE: ExpressionType;

    fn get_type(&self) -> ExpressionType;

    fn has_aggregation(&self) -> bool;

    fn has_window_function(&self) -> bool {
        false
    }
}

impl Binder<'_> {
    pub(crate) fn parse_expression(&self, node: &sqlparser::ast::Expr) -> Result<ExpressionTypeImpl, TryFromASTError> {
        todo!()
        // if node.node.is_none() {
        //     return Err(TryFromASTError::FailedParsing("Node is none".to_string()))
        // }
        //
        //
        //
        // match node.node.as_ref().unwrap() {
        //     _ => unimplemented!()
        // }
    }
}
