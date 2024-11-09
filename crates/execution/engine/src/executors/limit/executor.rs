use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use planner::{LimitPlanNode, PlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct LimitExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: &'a ExecutorContext<'a>,

    // ----

    /** The filter plan node to be executed */
    plan: &'a LimitPlanNode,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef<'a>,

    remaining: usize
}

impl<'a> LimitExecutor<'a> {
    pub(crate) fn new(child_executor: ExecutorRef<'a>, plan: &'a LimitPlanNode, ctx: &'a ExecutorContext<'a>) -> Self {
        Self {
            remaining: plan.get_limit(),
            plan,
            child_executor,
            ctx,
        }
    }
}

impl Debug for LimitExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Limit").field("iter", &self.child_executor).finish()
    }
}


impl Iterator for LimitExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None
        }
        let item = self.child_executor.next()?;

        self.remaining -= 1;

        Some(item)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}

impl ExecutorMetadata for LimitExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for LimitExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::Limit(self)
    }
}

impl<'a> Executor<'a> for LimitExecutor<'a> {}
