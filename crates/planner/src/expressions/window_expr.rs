use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::WindowExpr;
use expression::ExpressionRef;
use std::rc::Rc;

impl PlanExpression for WindowExpr {
    fn plan<'a>(&self, _children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        unreachable!("should not parse window expressions here")
    }
}
