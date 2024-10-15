use binder::DeleteStatement;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl Plan for DeleteStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        let table = self.table.plan(planner);

        todo!();
    }
}
