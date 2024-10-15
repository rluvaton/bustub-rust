use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::TableReferenceTypeImpl;

impl Plan for TableReferenceTypeImpl {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        match self {
            TableReferenceTypeImpl::Invalid => panic!("Invalid table ref"),
            TableReferenceTypeImpl::BaseTable(t) => t.plan(planner),
            TableReferenceTypeImpl::Join(t) => t.plan(planner),
            TableReferenceTypeImpl::ExpressionList(t) => t.plan(planner),
            TableReferenceTypeImpl::CrossProduct(t) => t.plan(planner),
            TableReferenceTypeImpl::SubQuery(t) => t.plan(planner),
            TableReferenceTypeImpl::CTE(t) => t.plan(planner),
            TableReferenceTypeImpl::Empty => panic!("No plan for empty table ref")
        }
    }
}
