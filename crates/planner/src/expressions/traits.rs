use crate::plan_nodes::PlanType;
use crate::Planner;
use std::fmt::Debug;
use std::rc::Rc;

/// plan table ref
pub(crate) trait PlanExpression {
    fn plan<'a>(&self, children: Vec<Rc<PlanType>>, planner: &'a Planner<'a>) -> (String, Rc<PlanType>);
}
