use crate::context::ExecutorContext;
use crate::executors::iterator_ext::IteratorExt;
use crate::executors::{Executor, ExecutorRef, MockScanExecutor, ValuesExecutor};
use planner::PlanType;
use std::sync::Arc;

pub(crate) trait CreateExecutor<'a> {
    fn create_executor(&'a self, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;
}

impl<'a> CreateExecutor<'a> for PlanType {
    fn create_executor(&'a self, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        // call_each_variant!(self, p, {
        //     p.create_executor(ctx)
        // })
        match self {
            // PlanType::SeqScan(_) => {}
            // PlanType::Insert(_) => {}
            // PlanType::Delete(_) => {}
            // PlanType::Aggregation(_) => {}
            PlanType::Filter(f) => {
                let child = f.get_child_plan().create_executor(ctx.clone());

                child.filter_exec(f.clone(), ctx.clone())
            },
            // PlanType::Values(_) => {}
            PlanType::Projection(d) => {
                let child = d.get_child_plan().create_executor(ctx.clone());

                child.projection_exec(d.clone(), ctx.clone())
            }
            PlanType::Limit(l) => {
                let child = l.get_child_plan().create_executor(ctx.clone());

                child.limit_exec(l.clone(), ctx.clone())
            }
            PlanType::MockScan(_) => {
                MockScanExecutor::new(&self, ctx).into_ref()
            },
            PlanType::Values(p) => {
                ValuesExecutor::new(p, ctx).into_ref()
            },
            // PlanType::Window(_) => {}
            _ => unimplemented!("No executor found for the requested plan type {:#?}", self)
        }
    }
}
