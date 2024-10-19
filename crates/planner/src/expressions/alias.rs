use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::AliasExpr;
use expression::ExpressionRef;

impl PlanExpression for AliasExpr {
    fn plan<'a>(&self, children: &[&PlanType], planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        (self.alias.clone(), self.child.plan(children, planner).1)
    }
}
