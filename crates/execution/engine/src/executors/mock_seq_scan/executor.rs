use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorItem, ExecutorMetadata};
use catalog_schema::Schema;
use catalog_schema_mocks::{MockDataIterator, MockTableName};
use planner::{MockScanPlanNode, PlanNode, PlanType};
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct MockScanExecutor {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext>,

    // ----

    /** The plan node for the scan */
    plan: Rc<PlanType>,

    /** The cursor for the current mock scan */

    iter: MockDataIterator,
}

impl MockScanExecutor {
    pub(crate) fn new(plan: Rc<PlanType>, ctx: Arc<ExecutorContext>) -> Self {
        let mock_table_name: MockTableName = match &*plan {
            PlanType::MockScan(s) => s.get_table().as_str().try_into().expect("Must be a valid mock table name"),
            _ => unreachable!(),
        };

        Self {
            plan,
            ctx,
            iter: mock_table_name.get_data_iter(),
        }
    }
}

impl Debug for MockScanExecutor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MockScan").finish()
    }
}


impl Iterator for MockScanExecutor
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (values, rid) = self.iter.next()?;

        Some((
            Tuple::from_value(values, self.plan.get_output_schema().deref()),
            rid
        ))
    }
}

impl ExecutorMetadata for MockScanExecutor {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}


impl Executor for MockScanExecutor {}
