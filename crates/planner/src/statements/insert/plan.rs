use crate::plan_nodes::{PlanNode, PlanType};
use crate::traits::Plan;
use crate::{AggregationPlanNode, InsertPlan, Planner, ProjectionPlanNode};
use binder::InsertStatement;
use std::sync::Arc;

impl Plan for InsertStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        let select = self.select.plan(planner);

        let table_schema = self.get_table().schema.get_columns();

        let schema = select.get_output_schema();
        let child_schema = schema.get_columns();
        if table_schema.iter().zip(child_schema).any(|(col1, col2)| col1.get_type() != col2.get_type()) {
            panic!("table schema mismatch");
        }

        // TODO - fix this!, we should not prefix column names like this!
        let insert_schema = Arc::new(self.get_table().schema.prefix_column_names(self.get_table().table.as_str()));

        let mut plan = InsertPlan::new(
            insert_schema,
            select,
            self.get_table().oid,
        ).into();
        
        if !self.get_returning().is_empty() {
            plan = ProjectionPlanNode::create_from_returning(self.get_returning(), plan, planner).into();
        } else {
            plan = AggregationPlanNode::create_internal_result_count(plan, "insert_rows").into()
        }

        plan
    }
}
