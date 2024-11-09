use crate::context::ExecutorContext;
use crate::executors::{Executor, ExecutorImpl, ExecutorItem, ExecutorMetadata, ExecutorRef};
use catalog_schema::Schema;
use data_types::Value;
use expression::Expression;
use planner::{AggregationPlanNode, AggregationType, PlanNode};
use rid::RID;
use std::cmp::{max, min};
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use tuple::Tuple;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct AggregationExecutor<'a> {


    /// The executor context in which the executor runs
    ctx: &'a ExecutorContext<'a>,

    // ----

    /** The filter plan node to be executed */
    plan: &'a AggregationPlanNode,

    /** The child executor from which tuples are obtained */
    child_executor: ExecutorRef<'a>,

    finished: bool,
    
    initial_values: Vec<Value>,
    
    // Count rows
    // Count per column
    // Min
    // Max
    // Sum
    // count_all: usize,
    // count_per_expr: Vec<usize>,
    // min: Option<usize>,
    // max: Option<usize>,
    // sum: usize
}

impl<'a> AggregationExecutor<'a> {
    pub(crate) fn new(child_executor: ExecutorRef<'a>, plan: &'a AggregationPlanNode, ctx: &'a ExecutorContext<'a>) -> Self {
        let initial_values = plan.get_aggregate_types().iter()
            .zip(plan.get_aggregates().iter())
            .map(|(agg_type, agg)| {
                match agg_type {
                    AggregationType::CountStarAggregate | AggregationType::CountAggregate | AggregationType::SumAggregate => Value::from(0i64),
                    AggregationType::MinAggregate | AggregationType::MaxAggregate => Value::from(None::<i64>).try_cast_as(agg.get_return_type()).expect("should be able to cast"),
                }
            })
            .collect::<Vec<Value>>();
            
        Self {
            plan,
            child_executor,
            ctx,

            finished: false,
            
            initial_values,
            // 
            // count_all: 0,
            // count_per_expr,
            // min: None,
            // max: None,
            // sum: None,
        }
    }
}

impl Debug for AggregationExecutor<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Aggregation").field("iter", &self.child_executor).finish()
    }
}


impl Iterator for AggregationExecutor<'_>
{
    type Item = ExecutorItem;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        
        loop {
            let item = self.child_executor.next();
            
            if item.is_none() {
                break;
            }
            
            let item = item.unwrap();
            
            let agg_iter = self.plan
                .get_aggregate_types()
                .iter()
                .zip(
                    self.plan
                        .get_aggregates()
                        .iter()
                )
                .enumerate()
                .map(|(index, (agg_type, agg))| (index, agg_type, agg));
            
            for (index, agg_type, agg) in agg_iter {
                let agg_single_value = agg.evaluate(&item.0, self.child_executor.get_output_schema().deref());
                
                match agg_type {
                    // This does not use the agg
                    AggregationType::CountStarAggregate => {
                        self.initial_values[index] += Value::from(1i64);
                    }
                    AggregationType::CountAggregate => {
                        // If not null count the value
                        if !agg_single_value.is_null() {
                            self.initial_values[index] += Value::from(1i64);
                        }
                    }
                    AggregationType::SumAggregate => {
                        if !agg_single_value.is_null() {
                            self.initial_values[index] += agg_single_value;
                        }
                    }
                    AggregationType::MinAggregate => {
                        // If initial value is null than use the current value
                        if self.initial_values[index].is_null() {
                            self.initial_values[index] = agg_single_value;
                            
                            // If expression is not null as well calculate minimum
                        } else if !agg_single_value.is_null() {
                            // TODO - remove clone
                            self.initial_values[index] = min(agg_single_value, self.initial_values[index].clone());
                        }
                    }
                    AggregationType::MaxAggregate => {
                        // If initial value is null than use the current value
                        if self.initial_values[index].is_null() {
                            self.initial_values[index] = agg_single_value;

                            // If expression is not null as well calculate minimum
                        } else if !agg_single_value.is_null() {
                            // TODO - remove clone
                            self.initial_values[index] = max(agg_single_value, self.initial_values[index].clone());
                        }
                    }
                }
            }
        }
        
        self.finished = true;
        let tuple = Tuple::from_value(self.initial_values.as_slice(), self.plan.get_output_schema().deref());
        
        Some((tuple, RID::default()))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // This is one value thing
        (0, Some(1))
    }
}

impl ExecutorMetadata for AggregationExecutor<'_> {
    fn get_output_schema(&self) -> Arc<Schema> {
        self.plan.get_output_schema()
    }

    fn get_context(&self) -> &ExecutorContext {
        &self.ctx
    }
}

impl<'a> Into<ExecutorImpl<'a>> for AggregationExecutor<'a> {
    fn into(self) -> ExecutorImpl<'a> {
        ExecutorImpl::Aggregation(self)
    }
}

impl<'a> Executor<'a> for AggregationExecutor<'a> {

}
