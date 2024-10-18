use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata, FilterExecutor};
use catalog_schema::Schema;
use std::fmt::Display;
use std::sync::Arc;

// Helper to avoid duplicating deref on each variant
#[macro_export]
macro_rules! call_each_variant {
    ($enum_val:expr, $name:ident, $func:expr) => {
        match $enum_val {
            ExecutorImpl::Filter($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

#[derive(Debug)]
pub(crate) enum ExecutorImpl {
    // SeqScan,
    // IndexScan,
    // Insert(InsertPlan),
    // Update,
    // Delete(DeletePlan),
    // Aggregation(AggregationPlanNode),
    // Limit,
    // NestedLoopJoin,
    // NestedIndexJoin,
    // HashJoin,
    Filter(FilterExecutor),
    // Values(ValuesPlanNode),
    // Projection(ProjectionPlanNode),
    // Sort,
    // TopN,
    // TopNPerGroup,
    // MockScan,
    // InitCheck,
    // Window(WindowFunctionPlanNode)
}


impl ExecutorMetadata for ExecutorImpl {
    fn get_output_schema(&self) -> Arc<Schema> {
        call_each_variant!(self, e, {
            e.get_output_schema()
        })
    }

    fn get_context(&self) -> &ExecutorContext {
        call_each_variant!(self, e, {
            e.get_context()
        })
    }
}

impl Iterator for ExecutorImpl {
    type Item = ExecutorItem;

    fn next(&mut self) -> Option<Self::Item> {
        call_each_variant!(self, e, {
            e.next()
        })
    }
}

impl Executor for ExecutorImpl {}
