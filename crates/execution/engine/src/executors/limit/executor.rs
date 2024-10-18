use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use expression::Expression;
use planner::{FilterPlan, LimitPlanNode, PlanNode, ProjectionPlanNode};
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct LimitExecutor {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext>,

    // ----

    /** The filter plan node to be executed */
    plan: LimitPlanNode,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef,

    remaining: usize
}

impl LimitExecutor {
    pub(crate) fn new(child_executor: ExecutorRef, plan: LimitPlanNode, ctx: Arc<ExecutorContext>) -> Self {
        Self {
            remaining: plan.get_limit(),
            plan,
            child_executor,
            ctx,
        }
    }
}

impl Debug for LimitExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Limit").field("iter", &self.child_executor).finish()
    }
}


impl Iterator for LimitExecutor
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

impl<> ExecutorMetadata for LimitExecutor {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}


impl Executor for LimitExecutor {}
