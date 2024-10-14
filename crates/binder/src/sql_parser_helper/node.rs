use std::sync::Arc;
use crate::expressions::{Expression, ExpressionTypeImpl};
use crate::try_from_ast_error::ParseASTError;

pub(crate) trait NodeExt {
    fn parse_expression(&self) -> Result<Arc<ExpressionTypeImpl>, ParseASTError>;
}

// Node
impl NodeExt for () {
    fn parse_expression(&self) -> Result<Arc<ExpressionTypeImpl>, ParseASTError> {
        todo!()
        // if self.node.is_none() {
        //     return Err(TryFromASTError::FailedParsing("Node is none".to_string()))
        // }
        //
        // match self.node.as_ref().unwrap() {
        //     _ => unimplemented!()
        // }
    }
}
