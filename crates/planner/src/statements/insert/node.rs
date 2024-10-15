use crate::plan_nodes::{PlanNode, PlanType};
use catalog_schema::Schema;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct InsertPlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

}

impl InsertPlan {
    pub fn new(output: Arc<Schema>, child: PlanType) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
        }
    }
}

impl Display for InsertPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Insert")
            // .field("table_oid", &self.table_oid)
            .finish()
    }
}

impl PlanNode for InsertPlan {
    fn output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &Vec<PlanType> {
        &self.children
    }
}
