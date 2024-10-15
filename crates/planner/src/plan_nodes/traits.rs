use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use crate::plan_nodes::PlanType;

pub trait PlanNode: Clone + Display + Debug + Into<PlanType> {

    /** @return the schema for the output of this plan node */
    fn get_output_schema(&self) -> Arc<Schema>;

    /** @return the children of this plan node */
    fn get_children(&self) -> &[Rc<PlanType>];


    /** @return the child of this plan node at index child_idx */
    fn get_child_at(&self, child_idx: usize) -> &Rc<PlanType> {
        &self.get_children()[child_idx]
    }

    fn into_rc_plan_type(self) -> Rc<PlanType> {
        Rc::new(self.into())
    }
}

