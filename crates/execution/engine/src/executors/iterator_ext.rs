use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorRef, FilterExecutor, LimitExecutor, ProjectionExecutor};
use planner::{FilterPlan, LimitPlanNode, ProjectionPlanNode};
use std::sync::Arc;

pub trait IteratorExt<'a> {
    #[inline]
    fn filter_exec(self, plan: FilterPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[inline]
    fn projection_exec(self, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[inline]
    fn limit_exec(self, plan: LimitPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;
}

impl<'a> IteratorExt<'a> for ExecutorRef<'a> {
    #[inline]
    fn filter_exec(self, plan: FilterPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        FilterExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn projection_exec(self, plan: ProjectionPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        ProjectionExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn limit_exec(self, plan: LimitPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        LimitExecutor::new(self, plan, ctx).into_ref()
    }
}
