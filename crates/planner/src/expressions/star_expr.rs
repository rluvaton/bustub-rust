use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::StarExpr;
use expression::ExpressionRef;

impl PlanExpression for StarExpr {
    fn plan<'a>(&self, _children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        todo!()
    }
}
