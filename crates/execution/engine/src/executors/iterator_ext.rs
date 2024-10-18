use crate::context::ExecutorContext;
use crate::executors::{Executor, FilterExecutor, LimitExecutor, ProjectionExecutor};
use planner::{FilterPlan, LimitPlanNode, ProjectionPlanNode};
use std::sync::Arc;

pub trait IteratorExt {
    #[inline]
    fn filter_exec(self, plan: FilterPlan, ctx: Arc<ExecutorContext>) -> FilterExecutor;

    #[inline]
    fn projection_exec(self, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext>) -> ProjectionExecutor;

    #[inline]
    fn limit_exec(self, plan: LimitPlanNode, ctx: Arc<ExecutorContext>) -> LimitExecutor;
}

impl  IteratorExt for Box<dyn Executor> {
    #[inline]
    fn filter_exec(self, plan: FilterPlan, ctx: Arc<ExecutorContext>) -> FilterExecutor {
        FilterExecutor::new(self, plan, ctx)
    }

    #[inline]
    fn projection_exec(self, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext>) -> ProjectionExecutor
    {
        ProjectionExecutor::new(self, plan, ctx)
    }

    #[inline]
    fn limit_exec(self, plan: LimitPlanNode, ctx: Arc<ExecutorContext>) -> LimitExecutor {
        LimitExecutor::new(self, plan, ctx)
    }
}
