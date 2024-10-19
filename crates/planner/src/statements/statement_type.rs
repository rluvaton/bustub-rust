use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::StatementTypeImpl;

impl Plan for StatementTypeImpl {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        match self {
            StatementTypeImpl::Invalid => panic!("Invalid statement"),
            StatementTypeImpl::Select(node) => node.plan(planner),
            StatementTypeImpl::Insert(node) => node.plan(planner),
            StatementTypeImpl::Delete(node) => node.plan(planner),
            StatementTypeImpl::Create(_) => panic!("no plan needed for creation"),
        }
    }
}
