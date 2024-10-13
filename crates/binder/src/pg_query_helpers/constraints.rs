use pg_query::protobuf::a_const::Val;
use pg_query::protobuf::{ColumnDef, ConstrType, Constraint};
use pg_query::protobuf::node::Node;
use data_types::DBTypeId;
use db_core::catalog::Column;

pub(crate) trait ConstraintExt {
    fn is_primary_key(&self) -> bool;

    fn get_keys_names(&self) -> Vec<String>;
}


impl ConstraintExt for Constraint {
    fn is_primary_key(&self) -> bool {
        matches!(self.contype(), ConstrType::ConstrPrimary)
    }

    fn get_keys_names(&self) -> Vec<String> {
        self.keys
            .iter()
            .map(|node| {
                if node.node.is_none() {
                    unreachable!("Node cannot be missing {:#?}", node);
                }

                match &node.node.as_ref().unwrap() {
                    Node::String(str_node) => &str_node.sval,
                    _ => unreachable!("Unknown node type {:#?}", node),
                }
            })
            .cloned()
            .collect()
    }
}
