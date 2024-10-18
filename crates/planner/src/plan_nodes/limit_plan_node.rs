use crate::constants::UNNAMED_COLUMN;
use crate::plan_nodes::{PlanNode, PlanType};
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use expression::{Expression, ExpressionRef};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

/**
 * Limit constraints the number of output tuples produced by its child executor.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct LimitPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    // the limit
    limit: usize,
}

impl LimitPlanNode {

    /**
     * Construct a new LimitPlanNode instance.
     * @param child The child plan from which tuples are obtained
     * @param limit The number of output tuples
     */
    pub fn new(output: Arc<Schema>, child: PlanType, limit: usize) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            limit,
        }
    }


    /** @return limit */
    pub fn get_limit(&self) -> usize { self.limit }

    /** @return The child plan providing tuples to be limited */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "Limit should have exactly one child plan.");
        &self.children[0]
    }
}

impl Display for LimitPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Limit")
            .field("limit", &self.limit)
            .finish()
    }
}

impl Into<PlanType> for LimitPlanNode {
    fn into(self)-> PlanType {
        PlanType::Limit(self)
    }
}

impl PlanNode for LimitPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        &self.children
    }
}
