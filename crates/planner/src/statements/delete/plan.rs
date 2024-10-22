use crate::expressions::PlanExpression;
use crate::plan_nodes::{FilterPlan, PlanNode, PlanType};
use crate::traits::Plan;
use crate::{AggregationPlanNode, DeletePlan, Planner, ProjectionPlanNode};
use binder::DeleteStatement;
use std::sync::Arc;

impl Plan for DeleteStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        let table = self.get_table().plan(planner);

        let expr_children = vec![&table];

        let (_, condition) = self.filter_expr.plan(expr_children.as_slice(), planner);

        let filter = FilterPlan::new(table.get_output_schema(), condition, table);

        // TODO - fix this!, we should not prefix column names like this!
        let delete_schema = Arc::new(self.get_table().schema.prefix_column_names(self.get_table().table.as_str()));

        let mut plan = DeletePlan::new(delete_schema, filter.into(), self.get_table().oid).into();

        if !self.get_returning().is_empty() {
            plan = ProjectionPlanNode::create_from_returning(self.get_returning(), plan, planner).into();
        } else {
            plan = AggregationPlanNode::create_internal_result_count(plan, "delete_rows").into()
        }
        
        plan
    }
}
