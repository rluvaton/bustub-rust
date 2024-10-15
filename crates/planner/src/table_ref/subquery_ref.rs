use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::{ExpressionListRef, SubqueryRef};
use crate::traits::Plan;

impl Plan for SubqueryRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
