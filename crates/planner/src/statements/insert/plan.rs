use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::InsertStatement;
use std::rc::Rc;

impl Plan for InsertStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> Rc<PlanType> {
        todo!()
    }
}
