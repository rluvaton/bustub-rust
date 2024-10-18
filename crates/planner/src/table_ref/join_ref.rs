use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::JoinRef;
use crate::traits::Plan;

impl Plan for JoinRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        todo!()
    }
}
