use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use common::get_timestamp;
use db_core::catalog::TableInfo;
use planner::{InsertPlan, PlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use tuple::TupleMeta;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct InsertExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext<'a>>,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef<'a>,

    // ----

    plan: &'a InsertPlan,

    dest_table_info: Arc<TableInfo>,
}

impl<'a> InsertExecutor<'a> {
    pub(crate) fn new(child_executor: ExecutorRef<'a>, plan: &'a InsertPlan, ctx: Arc<ExecutorContext<'a>>) -> Self {
        let item = {
            let c = ctx.get_catalog().lock();

            c.get_table_by_oid(plan.get_table_oid()).expect("Table must exists (otherwise it should be blocked at the planner)")
        };

        Self {
            child_executor,
            plan,
            ctx,
            dest_table_info: item,
        }
    }
}

impl Debug for InsertExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Insert").field("child_executor", &self.child_executor).finish()
    }
}

impl Iterator for InsertExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (mut tuple, _) = self.child_executor.next()?;

        let rid = self.dest_table_info.get_table_heap().insert_tuple(
            &TupleMeta::new(
                get_timestamp(),
                false,
            ),
            &tuple,
            self.ctx.get_lock_manager(),
            self.ctx.get_transaction(),
            Some(self.plan.get_table_oid()),
        ).expect("Tuple is too big to fit in a page (this should be blocked in the planner)");

        tuple.set_rid(rid);
        
        // TODO - update indexes if relevant

        Some((tuple, rid))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.child_executor.size_hint()
    }
}

impl ExecutorMetadata for InsertExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for InsertExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::Insert(self)
    }
}

impl<'a> Executor<'a> for InsertExecutor<'a> {}
