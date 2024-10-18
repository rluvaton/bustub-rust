use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use expression::Expression;
use planner::{PlanNode, ProjectionPlanNode, ValuesPlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use rid::RID;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ValuesExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext<'a>>,

    // ----

    plan: &'a ValuesPlanNode,

    index: usize,

    dummy_tuple: Tuple,
    dummy_schema: Schema,
}

impl<'a> ValuesExecutor<'a> {
    pub(crate) fn new(plan: &'a ValuesPlanNode, ctx: Arc<ExecutorContext<'a>>) -> Self {
        Self {
            plan,
            ctx,
            index: 0,
            dummy_tuple: Tuple::new(RID::default()),
            dummy_schema: Schema::new(vec![])
        }
    }
}

impl Debug for ValuesExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Values").field("plan", &self.plan).finish()
    }
}


impl Iterator for ValuesExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.plan.get_values().len() {
            return None;
        }

        let values = self.plan.get_values()[self.index]
            .iter()
            .map(|col| col.evaluate(&self.dummy_tuple, &self.dummy_schema))
            .collect::<Vec<_>>();

        Some((
            Tuple::from_value(values, &*self.plan.get_output_schema()),

            // TODO - recheck this
            RID::default()
        ))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.plan.get_values().len().wrapping_sub(self.index);
        (size, Some(size))
    }
}

impl ExecutorMetadata for ValuesExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for ValuesExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::Values(self)
    }
}

impl<'a> Executor<'a> for ValuesExecutor<'a> {}
