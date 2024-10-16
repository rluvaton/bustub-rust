use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::AliasExpr;
use expression::ExpressionRef;
use std::rc::Rc;

impl PlanExpression for AliasExpr {
    fn plan<'a>(&self, children: &Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        (self.alias.clone(), self.child.plan(children, planner).1)
    }
}
