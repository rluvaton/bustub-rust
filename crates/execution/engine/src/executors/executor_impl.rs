use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata, FilterExecutor, LimitExecutor, MockScanExecutor, ProjectionExecutor, ValuesExecutor};
use catalog_schema::Schema;
use std::fmt::Display;
use std::sync::Arc;

// Helper to avoid duplicating deref on each variant
#[macro_export]
macro_rules! call_each_variant {
    ($enum_val:expr, $name:ident, $func:expr) => {
        match $enum_val {
            ExecutorImpl::Limit($name) => $func,
            ExecutorImpl::Filter($name) => $func,
            ExecutorImpl::Projection($name) => $func,
            ExecutorImpl::MockScan($name) => $func,
            ExecutorImpl::Values($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

#[derive(Debug)]
pub(crate) enum ExecutorImpl<'a> {
    // SeqScan,
    // IndexScan,
    // Insert(InsertPlan),
    // Update,
    // Delete(DeletePlan),
    // Aggregation(AggregationPlanNode),
    Limit(LimitExecutor<'a>),
    // NestedLoopJoin,
    // NestedIndexJoin,
    // HashJoin,
    Filter(FilterExecutor<'a>),
    Values(ValuesExecutor<'a>),
    Projection(ProjectionExecutor<'a>),
    // Sort,
    // TopN,
    // TopNPerGroup,
    MockScan(MockScanExecutor<'a>)
    // InitCheck,
    // Window(WindowFunctionPlanNode)
}


impl<'a> ExecutorMetadata for ExecutorImpl<'a> {
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

impl Iterator for ExecutorImpl<'_> {
    type Item = ExecutorItem;

    fn next(&mut self) -> Option<Self::Item> {
        call_each_variant!(self, e, {
            e.next()
        })
    }
}

impl<'a> Executor<'a> for ExecutorImpl<'a> {}
