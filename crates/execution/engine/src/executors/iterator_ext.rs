use crate::context::ExecutorContext;
use crate::executors::{DeleteExecutor, Executor, ExecutorRef, FilterExecutor, InsertExecutor, LimitExecutor, ProjectionExecutor};
use planner::{DeletePlan, FilterPlan, InsertPlan, LimitPlanNode, ProjectionPlanNode};
use std::sync::Arc;

pub trait IteratorExt<'a> {
    #[must_use]
    fn filter_exec(self, plan: &'a FilterPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[must_use]
    fn projection_exec(self, plan: &'a ProjectionPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[must_use]
    fn limit_exec(self, plan: &'a LimitPlanNode, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[must_use]
    fn insert_exec(self, plan: &'a InsertPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;

    #[must_use]
    fn delete_exec(self, plan: &'a DeletePlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;
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

    #[inline]
    fn insert_exec(self, plan: &'a InsertPlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        InsertExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn delete_exec(self, plan: &'a DeletePlan, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        DeleteExecutor::new(self, plan, ctx).into_ref()
    }
}
