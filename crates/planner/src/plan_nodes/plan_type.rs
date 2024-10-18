use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use crate::{AggregationPlanNode, DeletePlan, FilterPlan, HashJoinPlan, IndexScanPlanNode, InsertPlan, LimitPlanNode, MockScanPlanNode, NestedIndexJoinPlan, PlanNode, ProjectionPlanNode, SeqScanPlanNode, ValuesPlanNode, WindowFunctionPlanNode};

// Helper to avoid duplicating deref on each variant
macro_rules! call_each_variant {
    ($enum_val:expr, $name:ident, $func:expr) => {
        match $enum_val {
            PlanType::Insert($name) => $func,
            PlanType::Delete($name) => $func,
            PlanType::Filter($name) => $func,
            PlanType::Values($name) => $func,
            PlanType::Window($name) => $func,
            PlanType::Projection($name) => $func,
            PlanType::Aggregation($name) => $func,
            PlanType::MockScan($name) => $func,
            PlanType::SeqScan($name) => $func,
            PlanType::Limit($name) => $func,
            PlanType::HashJoin($name) => $func,
            PlanType::IndexScan($name) => $func,
            PlanType::NestedIndexJoin($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

/** PlanType represents the types of plans that we have in our system. */

#[derive(Clone, Debug, PartialEq)]
pub enum PlanType {
    SeqScan(SeqScanPlanNode),
    IndexScan(IndexScanPlanNode),
    Insert(InsertPlan),
    // Update,
    Delete(DeletePlan),
    Aggregation(AggregationPlanNode),
    Limit(LimitPlanNode),
    // NestedLoopJoin,
    NestedIndexJoin(NestedIndexJoinPlan),
    HashJoin(HashJoinPlan),
    Filter(FilterPlan),
    Values(ValuesPlanNode),
    Projection(ProjectionPlanNode),
    // Sort,
    // TopN,
    // TopNPerGroup,
    MockScan(MockScanPlanNode),
    // InitCheck,
    Window(WindowFunctionPlanNode)
}

impl Display for PlanType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        call_each_variant!(self, p, {
            p.fmt(f)
        })
    }
}

impl PlanNode for PlanType {
    fn get_output_schema(&self) -> Arc<Schema> {
        call_each_variant!(self, p, {
            p.get_output_schema()
        })
    }

    fn get_children(&self) -> &[Rc<PlanType>] {
        call_each_variant!(self, p, {
            p.get_children()
        })
    }

    fn get_child_at(&self, child_idx: usize) -> &Rc<PlanType> {
        call_each_variant!(self, p, {
            p.get_child_at(child_idx)
        })
    }
}
