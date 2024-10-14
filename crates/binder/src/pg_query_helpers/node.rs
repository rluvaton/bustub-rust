use std::sync::Arc;
use pg_query::Node;
use crate::expressions::Expression;
use crate::parse_node_error::ParsePgNodeError;

pub(crate) trait NodeExt {
    fn parse_expression(&self) -> Result<Arc<dyn Expression>, ParsePgNodeError>;
}

impl NodeExt for Node {
    fn parse_expression(&self) -> Result<Arc<dyn Expression>, ParsePgNodeError> {
        if self.node.is_none() {
            return Err(ParsePgNodeError::FailedParsing("Node is none".to_string()))
        }

        match self.node.as_ref().unwrap() {
            _ => unimplemented!()
        }
    }
}
