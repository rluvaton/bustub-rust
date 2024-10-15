use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use crate::plan_nodes::{FilterPlan, PlanNode};
use crate::statements::{DeletePlan, InsertPlan};

/** PlanType represents the types of plans that we have in our system. */

#[derive(Clone, Debug, PartialEq)]
pub enum PlanType {
    // SeqScan,
    // IndexScan,
    Insert(InsertPlan),
    // Update,
    Delete(DeletePlan),
    // Aggregation,
    // Limit,
    // NestedLoopJoin,
    // NestedIndexJoin,
    // HashJoin,
    Filter(FilterPlan),
    // Values,
    // Projection,
    // Sort,
    // TopN,
    // TopNPerGroup,
    // MockScan,
    // InitCheck,
    // Window
}

impl Display for PlanType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanType::Insert(p) => p.fmt(f),
            PlanType::Delete(p) => p.fmt(f),
            PlanType::Filter(p) => p.fmt(f),
        }
    }
}

impl PlanNode for PlanType {
    fn get_output_schema(&self) -> Arc<Schema> {
        match self {
            PlanType::Insert(p) => p.get_output_schema(),
            PlanType::Delete(p) => p.get_output_schema(),
            PlanType::Filter(p) => p.get_output_schema(),
        }
    }

    fn get_children(&self) -> &Vec<Rc<PlanType>> {
        match self {
            PlanType::Insert(p) => p.get_children(),
            PlanType::Delete(p) => p.get_children(),
            PlanType::Filter(p) => p.get_children(),
        }
    }

    fn get_child_at(&self, child_idx: usize) -> &Rc<PlanType> {
        match self {
            PlanType::Insert(p) => p.get_child_at(child_idx),
            PlanType::Delete(p) => p.get_child_at(child_idx),
            PlanType::Filter(p) => p.get_child_at(child_idx),
        }
    }
}
