use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use expression::Expression;
use planner::{PlanNode, ProjectionPlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ProjectionExecutor {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext>,

    // ----

    /** The filter plan node to be executed */
    plan: ProjectionPlanNode,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef,
}

impl ProjectionExecutor {
    pub(crate) fn new(child_executor: ExecutorRef, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext>) -> Self {
        Self {
            plan,
            child_executor,
            ctx,
        }
    }
}

impl Debug for ProjectionExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Projection").field("iter", &self.child_executor).finish()
    }
}


impl Iterator for ProjectionExecutor
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
        let (_, upper) = self.child_executor.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl<> ExecutorMetadata for ProjectionExecutor {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}


impl Executor for ProjectionExecutor {}
