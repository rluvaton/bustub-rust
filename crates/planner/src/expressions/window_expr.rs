use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{BinaryOpExpr, WindowExpr};
use crate::expressions::traits::PlanExpression;

impl PlanExpression for WindowExpr {
    fn plan<'a>(&self, children: Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (String, Rc<PlanType>) {
        todo!()
    }
}
