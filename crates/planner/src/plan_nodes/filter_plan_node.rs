use std::fmt::{Display, Formatter};
use std::sync::Arc;
use catalog_schema::Schema;
use common::config::TableOID;
use crate::plan_nodes::{PlanNode, PlanType};

/**
 * The FilterPlanNode represents a filter operation.
 * It retains any tuple that satisfies the predicate in the child.
 */
#[derive(Clone, Debug, PartialEq)]
pub struct FilterPlanNode {
    /**
     * The schema for the output of this plan node. In the volcano model, every plan node will spit out tuples,
     * and this tells you what schema this plan node's tuples will have.
     */
    output_schema: Arc<Schema>,

    /** The children of this plan node. */
    children: Vec<PlanType>,

    /** The predicate that all returned tuples must satisfy */
    predicate: PlanType,
}

impl FilterPlanNode {
    /**
     * Construct a new DeletePlanNode.
     * @param child The child plan to obtain tuple from
     * @param table_oid The identifier of the table from which tuples are deleted
     */
    pub fn new(output: Arc<Schema>, child: PlanType, predicate: PlanType) -> Self {
        Self {
            output_schema: output,
            children: vec![child],
            predicate,
        }
    }

    /** @return The predicate to test tuples against; tuples should only be returned if they evaluate to true */
    pub fn get(&self) -> &PlanType { &self.predicate }

    /** @return The child plan providing tuples to be deleted */
    pub fn get_child_plan(&self) -> &PlanType {
        assert_eq!(self.children.len(), 1, "filter should have exactly one child plan.");
        &self.children[0]
    }
}

impl Display for FilterPlanNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Filter")
            .field("predicate", &self.predicate)
            .finish()
    }
}

impl PlanNode for FilterPlanNode {
    fn output_schema(&self) -> Arc<Schema> {
        self.output_schema.clone()
    }

    fn get_children(&self) -> &Vec<PlanType> {
        &self.children
    }
}
