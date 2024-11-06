use std::ops::Deref;
use crate::plan_nodes::{PlanNode, PlanType};
use crate::traits::Plan;
use crate::{AggregationPlanNode, InsertPlan, Planner, ProjectionPlanNode};
use binder::InsertStatement;
use std::sync::Arc;
use data_types::CanBeCastedWithoutValueChangeResult;

impl Plan for InsertStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> error_utils::anyhow::Result<PlanType> {
        let select = self.get_select().plan(planner)?;

        let table_schema = self.get_table().schema.get_columns();

        let schema = select.get_output_schema();
        let child_schema = schema.get_columns();
        if !table_schema.iter().zip(child_schema).all(|(expected_schema_column, current_value_column)| {
            let cast_res = current_value_column
                .get_type()
                .can_be_cast_without_value_changes(&expected_schema_column.get_type());

            match cast_res {
                CanBeCastedWithoutValueChangeResult::True | CanBeCastedWithoutValueChangeResult::NeedBoundCheck => true,
                CanBeCastedWithoutValueChangeResult::False => false,
            }
        }) {
            // panic!("table schema mismatch");
            return Err(error_utils::anyhow!("internal: table schema mismatch"))
        }

        let mut insert_schema = self.get_table().schema.clone();

        let column_ordering = self.get_column_ordering();

        if column_ordering.should_use_column_ordering() {
            insert_schema = Arc::new(column_ordering.create_new_schema(insert_schema.deref()));
        }

        // TODO - fix this!, we should not prefix column names like this!
        insert_schema = Arc::new(insert_schema.prefix_column_names(self.get_table().table.as_str()));

        let mut plan = InsertPlan::new(
            insert_schema,
            select,
            column_ordering.clone(),
            self.get_table().oid,
        ).into();
        
        if !self.get_returning().is_empty() {
            plan = ProjectionPlanNode::create_from_returning(self.get_returning(), plan, planner).into();
        } else {
            plan = AggregationPlanNode::create_internal_result_count(plan, "insert_rows").into()
        }

        Ok(plan)
    }
}
