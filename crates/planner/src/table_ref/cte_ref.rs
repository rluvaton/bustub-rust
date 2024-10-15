use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::CTERef;

impl Plan for CTERef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> Rc<PlanType> {
        todo!()
    }
}
