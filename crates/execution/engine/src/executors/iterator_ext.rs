use std::sync::Arc;
use crate::context::ExecutorContext;
use crate::executors::{create_filter, create_projection, Executor, ExecutorRef, FilterExecutor, ProjectionExecutor};
use planner::{FilterPlan, ProjectionPlanNode};

pub trait IteratorExt {
    #[inline]
    fn filter_exec(self, plan: FilterPlan, ctx: Arc<ExecutorContext>) -> FilterExecutor;

    #[inline]
    fn projection_exec(self, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext>) -> ProjectionExecutor;
}

impl  IteratorExt for Box<dyn Executor> {
    #[inline]
    fn filter_exec(self, plan: FilterPlan, ctx: Arc<ExecutorContext>) -> FilterExecutor
    {
        create_filter(self, plan, ctx)
    }

    #[inline]
    fn projection_exec(self, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext>) -> ProjectionExecutor
    {
        create_projection(self, plan, ctx)
    }
}
