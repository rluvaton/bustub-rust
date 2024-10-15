use binder::ExpressionTypeImpl;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl Plan for ExpressionTypeImpl {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        match self {
            ExpressionTypeImpl::ColumnRef(e) => e.plan(planner),
            ExpressionTypeImpl::Constant(e) => e.plan(planner),
            ExpressionTypeImpl::Alias(e) => e.plan(planner),
            ExpressionTypeImpl::BinaryOp(e) => e.plan(planner),
            ExpressionTypeImpl::UnaryOp(e) => e.plan(planner),
            ExpressionTypeImpl::Star(e) => e.plan(planner),
            ExpressionTypeImpl::FuncCall(e) => e.plan(planner),
            ExpressionTypeImpl::AggCall(e) => e.plan(planner),
            ExpressionTypeImpl::Window(e) => e.plan(planner),
            ExpressionTypeImpl::Invalid => panic!("Invalid expression when trying to plan")
        }
    }
}
