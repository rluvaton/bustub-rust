use data_types::DBTypeId;
use db_core::catalog::Column;

pub(crate) trait ConstraintExt {
    fn is_primary_key(&self) -> bool;

    fn get_keys_names(&self) -> Vec<String>;
}


// Constraint
impl ConstraintExt for () {
    fn is_primary_key(&self) -> bool {
        todo!()
        // matches!(self.contype(), ConstrType::ConstrPrimary)
    }

    fn get_keys_names(&self) -> Vec<String> {
        todo!()
        //
        // self.keys
        //     .iter()
        //     .map(|node| {
        //         if node.node.is_none() {
        //             unreachable!("Node cannot be missing {:#?}", node);
        //         }
        //
        //         match &node.node.as_ref().unwrap() {
        //             Node::String(str_node) => &str_node.sval,
        //             _ => unreachable!("Unknown node type {:#?}", node),
        //         }
        //     })
        //     .cloned()
        //     .collect()
    }
}
