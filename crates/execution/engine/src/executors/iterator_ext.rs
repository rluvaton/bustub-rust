use std::iter::Filter;
use planner::FilterPlan;
use crate::context::ExecutorContext;
use crate::executors::{create_filter, Executor, ExecutorItem, FilterExecutor};

trait IteratorExt {
    #[inline]
    fn filter_exec<E>(self, plan: FilterPlan, ctx: ExecutorContext) -> FilterExecutor<Self>
    where
        Self: Sized + Executor,
    {
        create_filter(self, plan, ctx)
    }
}

impl<E: Executor> IteratorExt for E {

}
