use std::rc::Rc;
use std::sync::Arc;
use binder::DeleteStatement;
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use crate::expressions::PlanExpression;
use crate::plan_nodes::{FilterPlan, PlanNode, PlanType};
use crate::Planner;
use crate::statements::DeletePlan;
use crate::traits::Plan;

impl Plan for DeleteStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>) -> Rc<PlanType> {
        let table = self.get_table().plan(planner);

        let (_, condition) = self.expr.plan(vec![table.clone()], planner);

        let filter = FilterPlan::new(table.get_output_schema(), condition.into(), table);
        let delete_schema = Arc::new(Schema::new(vec![
            Column::new_fixed_size("__bustub_internal.delete_rows".to_string(), DBTypeId::INT)
        ]));

        Rc::new(DeletePlan::new(delete_schema, Rc::new(filter.into()), self.get_table().oid).into())
    }
}
