use binder::DeleteStatement;
use crate::expressions::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use crate::traits::Plan;

impl Plan for DeleteStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        let table = self.table.plan(planner);

        let (_, condition) = self.expr.plan(vec![table], planner);

        todo!();
    }
}
