use std::fmt::Debug;
use std::sync::Arc;
use pg_query::protobuf::node::Node;
use crate::Binder;
use crate::expressions::{ColumnRef, ExpressionType, ExpressionTypeImpl};
use crate::parse_node_error::ParsePgNodeError;

pub(crate) trait Expression: Debug + PartialEq {
    const TYPE: ExpressionType;

    fn get_type(&self) -> ExpressionType;

    fn has_aggregation(&self) -> bool;

    fn has_window_function(&self) -> bool {
        false
    }
}

impl Binder {
    pub(crate) fn parse_expression(&self, node: &pg_query::protobuf::Node) -> Result<ExpressionTypeImpl, ParsePgNodeError> {
        if node.node.is_none() {
            return Err(ParsePgNodeError::FailedParsing("Node is none".to_string()))
        }



        match node.node.as_ref().unwrap() {
            _ => unimplemented!()
        }
    }
}
