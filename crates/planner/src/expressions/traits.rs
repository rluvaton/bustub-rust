use crate::plan_nodes::PlanType;
use crate::Planner;
use expression::ExpressionRef;

/// plan table ref
pub(crate) trait PlanExpression {
    fn plan<'a>(&self, children: &[&PlanType], planner: &'a Planner<'a>) -> (String, ExpressionRef);
}
