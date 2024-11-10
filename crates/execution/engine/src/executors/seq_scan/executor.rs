use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata};
use catalog_schema::Schema;
use planner::{PlanNode, SeqScanPlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use table::TableIterator;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct SeqScanExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: &'a ExecutorContext<'a>,

    // ----

    /** The plan node for the scan */
    plan: &'a SeqScanPlanNode,

    iter: TableIterator<'a>,
}

impl<'a> SeqScanExecutor<'a> {
    pub(crate) fn new(plan: &'a SeqScanPlanNode, ctx: &'a ExecutorContext<'a>) -> SeqScanExecutor<'a> {
        let iter = ctx
            .get_catalog()
            .get_table_by_oid(plan.get_table_oid())
            .expect("Table must exists (if table is missing it should be stopped at the planner)")
            .get_table_heap()
            .iter();
        
        Self {
            plan,
            iter,
            ctx,
        }
    }
}

impl Debug for SeqScanExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SeqScan").finish()
    }
}


impl Iterator for SeqScanExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // TODO - support predicate in seq scan
        let (_, tuple) = self.iter.next()?;
        
        let rid = *tuple.get_rid();

        Some(Ok((tuple, rid)))
    }
}

impl ExecutorMetadata for SeqScanExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for SeqScanExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::SeqScan(self)
    }
}

impl<'a> Executor<'a> for SeqScanExecutor<'a> {}
