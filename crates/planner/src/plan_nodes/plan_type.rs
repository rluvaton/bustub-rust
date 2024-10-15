use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::sync::Arc;
use catalog_schema::Schema;
use crate::plan_nodes::{FilterPlan, PlanNode, ProjectionPlanNode, ValuesPlanNode};
use crate::plan_nodes::window_plan_node::WindowFunctionPlanNode;
use crate::statements::{DeletePlan, InsertPlan};

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
            // Add match arms for other variants as necessary
        }
    };
}

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
    Values(ValuesPlanNode),
    Projection(ProjectionPlanNode),
    // Sort,
    // TopN,
    // TopNPerGroup,
    // MockScan,
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
