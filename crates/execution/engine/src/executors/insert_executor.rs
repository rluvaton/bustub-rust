use index::Index;
use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use common::get_timestamp;
use db_core::catalog::{IndexInfo, TableInfo};
use planner::{InsertPlan, PlanNode};
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use tuple::{Tuple, TupleMeta};

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct InsertExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: Arc<ExecutorContext<'a>>,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef<'a>,

    // ----

    plan: &'a InsertPlan,

    // Saved here to avoid re-computing each time
    should_use_column_ordering_and_default_values: bool,

    // The table info for the table the values should be inserted into
    dest_table_info: Arc<TableInfo>,
    
    // The indexes of the matching dest table
    dest_indexes: Vec<Arc<IndexInfo>>
}

impl<'a> InsertExecutor<'a> {
    pub(crate) fn new(child_executor: ExecutorRef<'a>, plan: &'a InsertPlan, ctx: Arc<ExecutorContext<'a>>) -> Self {
        let (dest_table_info, dest_indexes) = {
            let c = ctx.get_catalog().lock();

            let table_info = c.get_table_by_oid(plan.get_table_oid()).expect("Table must exists (otherwise it should be blocked at the planner)");
            
            let indexes = c.get_table_indexes_by_name(table_info.get_name());

            (table_info, indexes)
        };

        Self {
            child_executor,
            plan,
            should_use_column_ordering_and_default_values: plan.get_column_ordering_and_default_values().should_use_column_ordering(),
            ctx,
            dest_table_info,
            dest_indexes
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

        // Fill default values and/or reorder columns to match the table order
        if self.should_use_column_ordering_and_default_values {
            let columns_ordering_and_default_values = self.plan.get_column_ordering_and_default_values();

            let table_schema = self.dest_table_info.get_schema();
            let values_to_insert = columns_ordering_and_default_values.map_values_based_on_schema(
                table_schema.deref(),
                tuple.get_values(self.plan.get_output_schema().deref()).deref(),
                |col| unimplemented!()
            );

            tuple = Tuple::from_value(values_to_insert.as_slice(), table_schema.deref())
        }

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
        
        // Update indexes
        self.dest_indexes
            .iter()
            .for_each(|index_info| {
                let index = index_info.get_index();
                
                index
                    .insert_entry(&tuple, rid,  self.ctx.get_transaction())
                    .expect("Should insert to index");
            });

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
