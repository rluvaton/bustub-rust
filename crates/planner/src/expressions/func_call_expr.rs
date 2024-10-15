use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOpExpr, FuncCallExpr};
use crate::expressions::traits::PlanExpression;

impl PlanExpression for FuncCallExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> (String, PlanType) {
        todo!()
    }
}
