use std::rc::Rc;
use binder::AliasExpr;
use expression::ExpressionRef;
use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl PlanExpression for AliasExpr {
    fn plan<'a>(&self, children: Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        todo!()
    }
}
