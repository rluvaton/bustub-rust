use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata};
use catalog_schema::Schema;
use catalog_schema_mocks::{MockDataIterator, MockTableName};
use planner::{MockScanPlanNode, PlanNode};
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct MockScanExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: &'a ExecutorContext<'a>,

    // ----

    /** The plan node for the scan */
    plan: &'a MockScanPlanNode,

    /** The cursor for the current mock scan */

    iter: MockDataIterator,
}

impl<'a> MockScanExecutor<'a> {
    pub(crate) fn new(plan: &'a MockScanPlanNode, ctx: &'a ExecutorContext<'a>) -> Self {
        let mock_table_name: MockTableName = plan.get_table().as_str().try_into().expect("Must be a valid mock table name");

        Self {
            plan,
            ctx,
            iter: mock_table_name.get_data_iter(),
        }
    }
}

impl Debug for MockScanExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockScan").finish()
    }
}


impl Iterator for MockScanExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (values, rid) = self.iter.next()?;

        Some((
            Tuple::from_value(values.as_slice(), self.plan.get_output_schema().deref()),
            rid
        ))
    }
}

impl ExecutorMetadata for MockScanExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for MockScanExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::MockScan(self)
    }
}

impl<'a> Executor<'a> for MockScanExecutor<'a> {}
