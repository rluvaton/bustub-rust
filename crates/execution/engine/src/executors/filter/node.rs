use std::sync::Arc;
use planner::FilterPlan;
use crate::context::ExecutorContext;
use crate::executors::{CreateExecutor, ExecutorRef};

impl CreateExecutor for FilterPlan {
    fn create_executor(&self, ctx: Arc<ExecutorContext>) -> ExecutorRef {
        todo!()
    }
}
