use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOpExpr, UnaryOpExpr};

impl Plan for UnaryOpExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
