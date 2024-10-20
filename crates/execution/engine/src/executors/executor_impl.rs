use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata, FilterExecutor, InsertExecutor, LimitExecutor, MockScanExecutor, ProjectionExecutor, SeqScanExecutor, ValuesExecutor};
use catalog_schema::Schema;
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
            ExecutorImpl::Insert($name) => $func,
            ExecutorImpl::SeqScan($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}

#[derive(Debug)]
#[must_use]
pub(crate) enum ExecutorImpl<'a> {
    SeqScan(SeqScanExecutor<'a>),
    // IndexScan,
    Insert(InsertExecutor<'a>),
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
