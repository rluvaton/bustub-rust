use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::StarExpr;

impl Plan for StarExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
