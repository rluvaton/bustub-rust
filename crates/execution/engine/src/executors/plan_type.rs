use crate::context::ExecutorContext;
use crate::executors::iterator_ext::IteratorExt;
use crate::executors::{Executor, ExecutorRef, MockScanExecutor, SeqScanExecutor, ValuesExecutor};
use planner::{PlanNode, PlanType};
use std::sync::Arc;

pub(crate) trait CreateExecutor<'a> {
    fn create_executor(&'a self, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a>;
}

impl<'a> CreateExecutor<'a> for PlanType {
    fn create_executor(&'a self, ctx: Arc<ExecutorContext<'a>>) -> ExecutorRef<'a> {
        match self {
            PlanType::SeqScan(plan) => {
                assert_eq!(plan.get_children(), &[], "SeqScan must not have any children");
                SeqScanExecutor::new(plan, ctx).into_ref()
            }
            PlanType::Insert(plan) => {
                let child = plan.get_child_plan().create_executor(ctx.clone());

                child.insert_exec(plan, ctx.clone())
            }
            PlanType::Delete(plan) => {
                let child = plan.get_child_plan().create_executor(ctx.clone());

                child.delete_exec(plan, ctx.clone())
            }
            PlanType::Aggregation(plan) => {
                let child = plan.get_child_plan().create_executor(ctx.clone());
                
                child.aggregation_exec(plan, ctx.clone())
            }
            PlanType::Filter(plan) => {
                let child = plan.get_child_plan().create_executor(ctx.clone());

                child.filter_exec(plan, ctx.clone())
            },
            // PlanType::Values(_) => {}
            PlanType::Projection(plan) => {
                let child = plan.get_child_plan().create_executor(ctx.clone());

                child.projection_exec(plan, ctx.clone())
            }
            PlanType::Limit(plan) => {
                let child = plan.get_child_plan().create_executor(ctx.clone());

                child.limit_exec(plan, ctx.clone())
            }
            PlanType::MockScan(plan) => {
                assert_eq!(plan.get_children(), &[], "Mock scan must not have any children");
                MockScanExecutor::new(plan, ctx).into_ref()
            },
            PlanType::Values(plan) => {
                assert_eq!(plan.get_children(), &[], "Values must not have any children");
                
                ValuesExecutor::new(plan, ctx).into_ref()
            },
            // PlanType::Window(_) => {}
            
            // This should be blocked in the planner
            _ => unreachable!("No executor found for the requested plan type {:#?}", self)
        }
    }
}
