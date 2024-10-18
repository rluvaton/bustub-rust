use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::ExpressionListRef;

impl Plan for ExpressionListRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        todo!()
    }
}
