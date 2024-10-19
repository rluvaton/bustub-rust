use crate::expressions::PlanExpression;
use crate::plan_nodes::{FilterPlan, PlanNode, PlanType};
use crate::traits::Plan;
use crate::{DeletePlan, Planner};
use binder::DeleteStatement;
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use std::sync::Arc;

impl Plan for DeleteStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> PlanType {
        let table = self.get_table().plan(planner);

        let expr_children = vec![&table];

        let (_, condition) = self.expr.plan(expr_children.as_slice(), planner);

        let filter = FilterPlan::new(table.get_output_schema(), condition, table);
        let delete_schema = Arc::new(Schema::new(vec![
            Column::new_fixed_size("__bustub_internal.delete_rows".to_string(), DBTypeId::INT)
        ]));

        DeletePlan::new(delete_schema, filter.into(), self.get_table().oid).into()
    }
}
