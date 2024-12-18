use crate::context::ExecutorContext;
use crate::executors::{DeleteExecutor, Executor, ExecutorRef, FilterExecutor, InsertExecutor, LimitExecutor, ProjectionExecutor};
use planner::{AggregationPlanNode, DeletePlan, FilterPlan, InsertPlan, LimitPlanNode, ProjectionPlanNode};
use crate::executors::aggregations::AggregationExecutor;

pub trait IteratorExt<'a> {
    #[must_use]
    fn filter_exec(self, plan: &'a FilterPlan, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a>;

    #[must_use]
    fn projection_exec(self, plan: &'a ProjectionPlanNode, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a>;

    #[must_use]
    fn limit_exec(self, plan: &'a LimitPlanNode, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a>;

    #[must_use]
    fn insert_exec(self, plan: &'a InsertPlan, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a>;

    #[must_use]
    fn delete_exec(self, plan: &'a DeletePlan, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a>;

    #[must_use]
    fn aggregation_exec(self, plan: &'a AggregationPlanNode, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a>;
}

impl<'a> IteratorExt<'a> for ExecutorRef<'a> {
    #[inline]
    fn filter_exec(self, plan: &'a FilterPlan, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a> {
        FilterExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn projection_exec(self, plan: &'a ProjectionPlanNode, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a> {
        ProjectionExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn limit_exec(self, plan: &'a LimitPlanNode, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a> {
        LimitExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn insert_exec(self, plan: &'a InsertPlan, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a> {
        InsertExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn delete_exec(self, plan: &'a DeletePlan, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a> {
        DeleteExecutor::new(self, plan, ctx).into_ref()
    }

    #[inline]
    fn aggregation_exec(self, plan: &'a AggregationPlanNode, ctx: &'a ExecutorContext<'a>) -> ExecutorRef<'a> {
        AggregationExecutor::new(self, plan, ctx).into_ref()
    }
}
