use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use expression::Expression;
use planner::{FilterPlan, PlanNode};
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use data_types::BooleanType;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct FilterExecutor {


    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext>,

    // ----

    /** The filter plan node to be executed */
    plan: FilterPlan,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef,
}

impl FilterExecutor {
    pub(crate) fn new(child_executor: ExecutorRef, plan: FilterPlan, ctx: Arc<ExecutorContext>) -> FilterExecutor {
        Self {
            plan,
            child_executor,
            ctx,
        }
    }
}

impl Debug for FilterExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Filter").field("iter", &self.child_executor).finish()
    }
}


impl Iterator for FilterExecutor
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let filter_expr = self.plan.get_predicate();
        let output_schema = self.child_executor.get_output_schema();

        self.child_executor.find(move |(tuple, _) | {
            let value = filter_expr.evaluate(tuple, output_schema.deref());

            value.try_into().is_ok_and(|val: BooleanType| val.get_as_bool().is_some_and(|b| b))
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.child_executor.size_hint();
        (0, upper) // can't know a lower bound, due to the predicate
    }
}

impl ExecutorMetadata for FilterExecutor {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}



impl Executor for FilterExecutor {

}
