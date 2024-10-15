use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::SelectStatement;

impl Plan for SelectStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> Rc<PlanType> {
        todo!()
    }
}
