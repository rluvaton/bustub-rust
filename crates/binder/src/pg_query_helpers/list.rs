use std::sync::Arc;
use crate::expressions::{Expression, ExpressionType, ExpressionTypeImpl};
use crate::try_from_ast_error::TryFromASTError;
use crate::pg_query_helpers::node::NodeExt;

pub(crate) trait ListExt {

    fn parse_items_as_expressions(&self) -> Result<Vec<Arc<ExpressionTypeImpl>>, TryFromASTError>;
}

// List
impl ListExt for () {
    fn parse_items_as_expressions(&self) -> Result<Vec<Arc<ExpressionTypeImpl>>, TryFromASTError> {
        todo!()
        // let mut select_list = vec![];
        //
        // for item in self.items {
        //     let expr = item.parse_expression()?;
        //
        //     if matches!(expr.get_type(), ExpressionType::Star) {
        //         return Err(TryFromASTError::Unimplemented("Unsupported * in expression list".to_string()))
        //     }
        //
        //     select_list.push(expr)
        // }
        //
        // Ok(select_list)
    }
}
