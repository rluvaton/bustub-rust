use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{AggCallExpr, BinaryOpExpr};

impl Plan for AggCallExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
