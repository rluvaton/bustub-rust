use binder::AliasExpr;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl Plan for AliasExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
