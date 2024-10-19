use crate::plan_nodes::PlanType;
use catalog_schema::Schema;
use std::fmt::{Debug, Display};
use std::sync::Arc;

// pub type PlanNodeRef = Rc<PlanType>;

pub(super) const EMPTY_CHILDREN: &'static [PlanType] = &[];

pub trait PlanNode: Clone + Display + Debug + Into<PlanType> {

    /** @return the schema for the output of this plan node */
    fn get_output_schema(&self) -> Arc<Schema>;

    /** @return the children of this plan node */
    fn get_children(&self) -> &[PlanType];


    /** @return the child of this plan node at index child_idx */
    fn get_child_at(&self, child_idx: usize) -> &PlanType {
        &self.get_children()[child_idx]
    }

    // TODO - remove this
    // fn into_ref(self) -> PlanType {
    //     self
    // }
}

