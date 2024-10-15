use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOpExpr, UnaryOpExpr};
use crate::expressions::traits::PlanExpression;

impl PlanExpression for UnaryOpExpr {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> (String, PlanType) {
        todo!()
    }
}
