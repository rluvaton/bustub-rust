use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::BinaryOpExpr;

impl Plan for BinaryOpExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
