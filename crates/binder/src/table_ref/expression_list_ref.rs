use crate::expressions::Expression;
use std::sync::Arc;
use pg_query::NodeRef;
use pg_query::protobuf::node::Node;
use crate::parse_node_error::ParsePgNodeError;
use crate::pg_query_helpers::ListExt;
use crate::table_ref::table_reference_type::TableReferenceType;
use crate::table_ref::TableRef;

/// A bound table ref type for `values` clause.
#[derive(Debug, PartialEq)]
pub struct ExpressionListRef {
    pub(crate) identifier: String,
    pub(crate) values: Vec<Vec<Arc<dyn Expression>>>,
}

impl ExpressionListRef {
    pub fn new(identifier: String, values: Vec<Vec<Arc<dyn Expression>>>) -> Self {
        Self {
            identifier,
            values,
        }
    }

    pub fn try_from_nodes(items: &Vec<pg_query::Node>) -> Result<Self, ParsePgNodeError> {
        let mut all_values: Vec<Vec<Arc<dyn Expression>>> = vec![];

        for node in &items {
            if node..node.is_none() {
                continue;
            }

            let node = node.node.as_ref().unwrap();

            let values = match node {
                Node::List(l) | Node::IntList(l) | Node::OidList(l) => l.parse_items_as_expressions()?,
                _ => return Err(ParsePgNodeError::FailedParsing("list item is not another list inside VALUES".to_string()))
            };

            if !all_values.is_empty() && all_values[0].len() != values.len() {
                return Err(ParsePgNodeError::FailedParsing("values must have the same length".to_string()));
            }

            all_values.push(values);
        }

        if all_values.is_empty() {
            return Err(ParsePgNodeError::FailedParsing("at least one row of values should be provided".to_string()))
        }

        Ok(ExpressionListRef::new("<unnamed>".to_string(), all_values))
    }
}

impl TableRef for ExpressionListRef {
    const TYPE: TableReferenceType = TableReferenceType::ExpressionList;
}

impl TryFrom<NodeRef<'_>> for ExpressionListRef {
    type Error = ParsePgNodeError;

    fn try_from(value: NodeRef) -> Result<Self, Self::Error> {
        let list = match value {
            NodeRef::List(list) => list,
            _ => return Err(ParsePgNodeError::IncompatibleType),
        };
        let nodes: &Vec<pg_query::Node> = &list.items;

        ExpressionListRef::try_from_nodes(nodes)
    }
}

