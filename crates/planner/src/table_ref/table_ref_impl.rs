use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::TableReferenceTypeImpl;

impl Plan for TableReferenceTypeImpl {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        todo!()
    }
}
