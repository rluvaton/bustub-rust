use crate::constants::UNNAMED_COLUMN;
use crate::plan_nodes::{PlanNode, PlanNodeRef, PlanType};
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use expression::{Expression, ExpressionRef};
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use binder::OrderByType;

/**
 * The SortPlanNode represents a sort operation. It will sort the input with
 * the given predicate.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct SortPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanNodeRef>,

    order_bys: Vec<(OrderByType, ExpressionRef)>,
}

impl SortPlanNode {

    /**
     * Construct a new SortPlanNode instance.
     * @param output The output schema of this sort plan node
     * @param child The child plan node
     * @param order_bys The sort expressions and their order by types.
     */
    pub fn new(output: Arc<Schema>, child: PlanNodeRef, order_bys: Vec<(OrderByType, ExpressionRef)>) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            order_bys,
        }
    }


    /** @return Get sort by expressions */
    pub fn get_order_by(&self) -> &[(OrderByType, ExpressionRef)] { self.order_bys.as_slice() }

    /** @return The child plan node */
    pub fn get_child_plan(&self) -> PlanNodeRef {
        assert_eq!(self.children.len(), 1, "Sort should have exactly one child plan.");
        self.children[0].clone()
    }
}

impl Display for SortPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sort")
            .field("order_bys", &self.order_bys)
            .finish()
    }
}

impl Into<PlanType> for SortPlanNode {
    fn into(self)-> PlanType {
        PlanType::Sort(self)
    }
}

impl PlanNode for SortPlanNode {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        &self.children
    }
}
