use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::ExpressionTypeImpl;
use expression::ExpressionRef;

impl PlanExpression for ExpressionTypeImpl {
    fn plan<'a>(&self, children: &[&PlanType], planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        match self {
            ExpressionTypeImpl::ColumnRef(e) => e.plan(children, planner),
            ExpressionTypeImpl::Constant(e) => e.plan(children, planner),
            ExpressionTypeImpl::Alias(e) => e.plan(children, planner),
            ExpressionTypeImpl::BinaryOp(e) => e.plan(children, planner),
            ExpressionTypeImpl::UnaryOp(e) => e.plan(children, planner),
            ExpressionTypeImpl::Star(e) => e.plan(children, planner),
            ExpressionTypeImpl::FuncCall(e) => e.plan(children, planner),
            ExpressionTypeImpl::AggCall(e) => e.plan(children, planner),
            ExpressionTypeImpl::Window(e) => e.plan(children, planner),
            ExpressionTypeImpl::Invalid => panic!("Invalid expression when trying to plan")
        }
    }
}
