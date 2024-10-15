use crate::plan_nodes::PlanNode;
use crate::Planner;

pub trait StatementPlan: PlanNode {
    type Statement: binder::Statement;

    fn create_node<'a>(statement: Self::Statement, planner: &'a Planner<'a>) -> Self;
}
