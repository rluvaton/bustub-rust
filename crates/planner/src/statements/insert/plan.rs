use std::rc::Rc;
use binder::{DeleteStatement, InsertStatement};
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl Plan for InsertStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> Rc<PlanType> {
        todo!()
    }
}
