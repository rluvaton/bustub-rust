use crate::plan_nodes::{PlanNode, PlanType};
use binder::OrderByType;
use catalog_schema::Schema;
use expression::ExpressionRef;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/**
 * The TopNPlanNode represents a top-n operation. It will gather the n extreme rows based on
 * limit and order expressions.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct TopNPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    order_bys: Vec<(OrderByType, ExpressionRef)>,

    // Retain N elements
    n: usize,
}

impl TopNPlanNode {

    /**
     * Construct a new TopNPlanNode instance.
     * @param output The output schema of this TopN plan node
     * @param child The child plan node
     * @param order_bys The sort expressions and their order by types.
     * @param n Retain n elements.
     */
    pub fn new(output: Arc<Schema>, child: PlanType, order_bys: Vec<(OrderByType, ExpressionRef)>, n: usize) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            order_bys,
            n,
        }
    }


    /** @return Get order by expressions */
    pub fn get_order_by(&self) -> &[(OrderByType, ExpressionRef)] { self.order_bys.as_slice() }

    /** @return The child plan node */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "TopN should have exactly one child plan.");
        &self.children[0]
    }

    /** @return The N (limit) */
    pub fn get_n(&self) -> usize { self.n }
}

impl Display for TopNPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TopNPlanNode")
            .field("n", &self.n)
            .field("order_bys", &self.order_bys)
            .finish()
    }
}

impl Into<PlanType> for TopNPlanNode {
    fn into(self)-> PlanType {
        PlanType::TopN(self)
    }
}

impl PlanNode for TopNPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[PlanType] {
        &self.children
    }
}
