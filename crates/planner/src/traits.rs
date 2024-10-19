use crate::plan_nodes::PlanType;
use crate::Planner;

/// plan table ref
pub(crate) trait Plan {

    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType;
}
