use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::UnaryOpExpr;
use expression::ExpressionRef;

impl PlanExpression for UnaryOpExpr {
    fn plan<'a>(&self, _children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        todo!()
    }
}
