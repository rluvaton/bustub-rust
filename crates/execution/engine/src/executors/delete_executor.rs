use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use common::get_timestamp;
use db_core::catalog::{IndexInfo, TableInfo};
use index::Index;
use planner::{DeletePlan, PlanNode};
use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;
use tuple::TupleMeta;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct DeleteExecutor<'a> {
    /// The executor context in which the executor runs
    ctx: &'a ExecutorContext<'a>,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef<'a>,

    // ----

    plan: &'a DeletePlan,

    // The table info for the table the values should be inserted into
    dest_table_info: &'a TableInfo,
    
    // The indexes of the matching dest table
    dest_indexes: Vec<&'a IndexInfo>
}

impl<'a> DeleteExecutor<'a> {
    pub(crate) fn new(child_executor: ExecutorRef<'a>, plan: &'a DeletePlan, ctx: &'a ExecutorContext<'a>) -> Self {
        let (dest_table_info, dest_indexes) = {
            let c = ctx.get_catalog();

            let table_info = c.get_table_by_oid(plan.get_table_oid()).expect("Table must exists (otherwise it should be blocked at the planner)");
            
            let indexes = c.get_table_indexes_by_name(table_info.get_name());

            (table_info, indexes)
        };

        Self {
            child_executor,
            plan,
            ctx,
            dest_table_info,
            dest_indexes
        }
    }
}

impl Debug for DeleteExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Delete").field("child_executor", &self.child_executor).finish()
    }
}

impl Iterator for DeleteExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        
        while let Some(res) = self.child_executor.next() {
            match res {
                Ok((tuple, rid)) => {
                    let is_deleted = self.dest_table_info.get_table_heap().mark_tuple_as_deleted(
                        &TupleMeta::new(
                            get_timestamp(),
                            true,
                        ),
                        &rid
                    );

                    // If not deleted just try the next one
                    if !is_deleted {
                        continue;
                    }

                    // Update indexes
                    self.dest_indexes
                        .iter()
                        .for_each(|index_info| {
                            let index = index_info.get_index();

                            index
                                .delete_entry(&tuple, rid,  self.ctx.get_transaction())
                                .expect("Should delete from index");
                        });

                    return Some(Ok((tuple, rid)));
                }
                Err(err) => {
                    return Some(Err(err))
                }
            }

        }
        
        None
        
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.child_executor.size_hint();
        (0, upper) // can't know a lower bound, due to the fact that some of the elements may be missing or already deleted
    }
}

impl ExecutorMetadata for DeleteExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for DeleteExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::Delete(self)
    }
}

impl<'a> Executor<'a> for DeleteExecutor<'a> {}
