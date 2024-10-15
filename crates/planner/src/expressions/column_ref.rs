use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::ColumnRef;
use expression::ExpressionRef;
use crate::expressions::traits::PlanExpression;

impl PlanExpression for ColumnRef {
    fn plan<'a>(&self, children: Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (Option<String>, ExpressionRef) {
        todo!()
    }
}
