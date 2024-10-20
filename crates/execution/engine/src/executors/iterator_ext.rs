use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorRef, FilterExecutor, LimitExecutor, ProjectionExecutor};
use planner::{FilterPlan, LimitPlanNode, ProjectionPlanNode};
use std::sync::Arc;

pub trait IteratorExt<'a> {
    #[must_use]
    fn filter_exec(self, plan: &'a FilterPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[must_use]
    fn projection_exec(self, plan: &'a ProjectionPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[must_use]
    fn limit_exec(self, plan: &'a LimitPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;
}

impl<'a> IteratorExt<'a> for ExecutorRef<'a> {
    #[inline]
    fn filter_exec(self, plan: &'a FilterPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        FilterExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn projection_exec(self, plan: &'a ProjectionPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        ProjectionExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn limit_exec(self, plan: &'a LimitPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        LimitExecutor::new(self, plan, ctx).into_ref()
    }
}
