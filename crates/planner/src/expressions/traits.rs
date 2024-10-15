use crate::plan_nodes::PlanType;
use crate::Planner;
use std::fmt::Debug;

/// plan table ref
pub(crate) trait PlanExpression {
    fn plan<'a>(&self, children: Vec<PlanType>, planner: &'a Planner<'a>) -> (String, PlanType);
}
