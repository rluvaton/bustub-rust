use crate::plan_nodes::{PlanNode, PlanType};
use std::fmt::Debug;
use std::rc::Rc;
use crate::Planner;

/// plan table ref
pub(crate) trait Plan {

    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType;
}
