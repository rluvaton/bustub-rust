use std::sync::Arc;
use pg_query::protobuf::{IntList, List, OidList};
use crate::expressions::{Expression, ExpressionType};
use crate::parse_node_error::ParsePgNodeError;
use crate::pg_query_helpers::node::NodeExt;

pub(crate) trait ListExt {

    fn parse_items_as_expressions(&self) -> Result<Vec<Arc<dyn Expression>>, ParsePgNodeError>;
}

impl ListExt for List {
    fn parse_items_as_expressions(&self) -> Result<Vec<Arc<dyn Expression>>, ParsePgNodeError> {
        let mut select_list = vec![];

        for item in self.items {
            let expr = item.parse_expression()?;

            if matches!(expr.get_type(), ExpressionType::Star) {
                return Err(ParsePgNodeError::Unimplemented("Unsupported * in expression list".to_string()))
            }

            select_list.push(expr)
        }

        Ok(select_list)
    }
}

impl ListExt for OidList {
    fn parse_items_as_expressions(&self) -> Result<Vec<Arc<dyn Expression>>, ParsePgNodeError> {
        let mut select_list = vec![];

        for item in self.items {
            let expr = item.parse_expression()?;

            if matches!(expr.get_type(), ExpressionType::Star) {
                return Err(ParsePgNodeError::Unimplemented("Unsupported * in expression list".to_string()))
            }

            select_list.push(expr)
        }

        Ok(select_list)
    }

}

impl ListExt for IntList {

    fn parse_items_as_expressions(&self) -> Result<Vec<Arc<dyn Expression>>, ParsePgNodeError> {
        let mut select_list = vec![];

        for item in self.items {
            let expr = item.parse_expression()?;

            if matches!(expr.get_type(), ExpressionType::Star) {
                return Err(ParsePgNodeError::Unimplemented("Unsupported * in expression list".to_string()))
            }

            select_list.push(expr)
        }

        Ok(select_list)
    }

}
