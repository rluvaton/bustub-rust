use crate::plan_nodes::{PlanNode, PlanType};
use crate::PlanNodeRef;
use catalog_schema::Schema;
use expression::ExpressionRef;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;

/**
 * The FilterPlanNode represents a filter operation.
 * It retains any tuple that satisfies the predicate in the child.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct FilterPlan {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<Rc<PlanType>>,

    /** The predicate that all returned tuples must satisfy */
    predicate: ExpressionRef,
}

impl FilterPlan {
    /**
     * Construct a new FilterPlanNode instance.
     * @param output The output schema of this filter plan node
     * @param predicate The predicate applied during the scan operation
     * @param child The child plan node
     */
    pub fn new(output: Arc<Schema>, predicate: ExpressionRef, child: Rc<PlanType>) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            predicate,
        }
    }

    /** @return The predicate to test tuples against; tuples should only be returned if they evaluate to true */
    pub fn get_predicate(&self) -> &ExpressionRef { &self.predicate }

    /** @return The child plan providing tuples to be deleted */
    pub fn get_child_plan(&self) -> PlanNodeRef {
        assert_eq!(self.children.len(), 1, "filter should have exactly one child plan.");
        self.children[0].clone()
    }
}

impl Display for FilterPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Filter")
            .field("predicate", &self.predicate)
            .finish()
    }
}

impl Into<PlanType> for FilterPlan {
    fn into(self)-> PlanType {
        PlanType::Filter(self)
    }
}

impl PlanNode for FilterPlan {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        &self.children
    }
}
