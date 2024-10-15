use binder::AliasExpr;
use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl PlanExpression for AliasExpr {
    fn plan<'a>(&self, children: Vec<PlanType>, planner: &'a Planner<'a>) -> (String, PlanType) {
        todo!()
    }
}
