use crate::context::ExecutorContext;
use crate::executors::{CreateExecutor, ExecutorRef};
use planner::PlanType;
use std::fmt::Display;
use std::sync::Arc;

// Helper to avoid duplicating deref on each variant
macro_rules! call_each_variant {
    ($enum_val:expr, $name:ident, $func:expr) => {
        match $enum_val {
            // PlanType::Insert($name) => $func,
            // PlanType::Delete($name) => $func,
            PlanType::Filter($name) => $func,
            // PlanType::Values($name) => $func,
            // PlanType::Window($name) => $func,
            // PlanType::Projection($name) => $func,
            // PlanType::Aggregation($name) => $func,
            _ => unimplemented!()
            // Add match arms for other variants as necessary
        }
    };
}

impl CreateExecutor for PlanType {
    fn create_executor(&self, ctx: Arc<ExecutorContext>) -> ExecutorRef {
        call_each_variant!(self, p, {
            p.create_executor(ctx)
        })
    }
}
