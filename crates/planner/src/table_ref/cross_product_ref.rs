use binder::CrossProductRef;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl Plan for CrossProductRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
