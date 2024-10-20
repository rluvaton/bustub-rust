use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use expression::Expression;
use planner::{PlanNode, ProjectionPlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ProjectionExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext<'a>>,

    // ----

    /** The filter plan node to be executed */
    plan: &'a ProjectionPlanNode,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef<'a>,
}

impl<'a> ProjectionExecutor<'a> {
    pub(crate) fn new(child_executor: ExecutorRef<'a>, plan: &'a ProjectionPlanNode, ctx: Arc<ExecutorContext<'a>>) -> Self {
        Self {
            plan,
            child_executor,
            ctx,
        }
    }
}

impl Debug for ProjectionExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Projection").field("child_executor", &self.child_executor).finish()
    }
}


impl Iterator for ProjectionExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (tuple, rid) = self.child_executor.next()?;

        let values = self.plan.get_expressions()
            .iter()
            .map(|expr| expr.evaluate(&tuple, &*self.child_executor.get_output_schema().clone()))
            .collect::<Vec<_>>();

        Some((
            Tuple::from_value(values, &*self.plan.get_output_schema()),
            rid
        ))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.child_executor.size_hint()
    }
}

impl ExecutorMetadata for ProjectionExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for ProjectionExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::Projection(self)
    }
}

impl<'a> Executor<'a> for ProjectionExecutor<'a> {}
