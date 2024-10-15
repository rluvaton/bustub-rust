use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{ColumnRef, Constant};

impl Plan for Constant {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
