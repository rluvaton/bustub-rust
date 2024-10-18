use crate::plan_nodes::{PlanNode, PlanType};
use crate::traits::Plan;
use crate::{InsertPlan, Planner};
use binder::InsertStatement;
use std::sync::Arc;
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;

impl Plan for InsertStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        let select = self.select.plan(planner);

        let table_schema = self.get_table().schema.get_columns();

        let schema = select.get_output_schema();
        let child_schema = schema.get_columns();
        if table_schema.iter().zip(child_schema).any(|(col1, col2)| col1.get_type() != col2.get_type()) {
            panic!("table schema mismatch");
        }

        let insert_schema = Schema::new(vec![
            Column::new_fixed_size("__bustub_internal.insert_rows".to_string(), DBTypeId::INT)
        ]);

        InsertPlan::new(
            Arc::new(insert_schema),
            select,
            self.get_table().oid
        ).into()
    }
}
