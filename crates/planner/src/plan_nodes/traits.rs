use std::fmt::{Debug, Display};
use std::sync::Arc;
use catalog_schema::Schema;
use crate::plan_nodes::PlanType;

pub trait PlanNode: Clone + Display + Debug {

    /** @return the schema for the output of this plan node */
    fn output_schema(&self) -> Arc<Schema>;

    /** @return the children of this plan node */
    fn get_children(&self) -> &Vec<PlanType>;


    /** @return the child of this plan node at index child_idx */
    fn get_child_at(&self, child_idx: usize) -> &PlanType {
        &self.get_children()[child_idx]
    }
}

