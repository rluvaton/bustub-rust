use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOpExpr, FuncCallExpr};

impl Plan for FuncCallExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
