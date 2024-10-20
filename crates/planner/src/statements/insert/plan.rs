use crate::plan_nodes::{PlanNode, PlanType};
use crate::traits::Plan;
use crate::{InsertPlan, Planner, ProjectionPlanNode};
use binder::InsertStatement;
use std::sync::Arc;
use catalog_schema::{Column, Schema};
use data_types::DBTypeId;
use expression::ExpressionRef;
use crate::expressions::PlanExpression;

impl Plan for InsertStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        let select = self.select.plan(planner);

        let table_schema = self.get_table().schema.get_columns();

        let schema = select.get_output_schema();
        let child_schema = schema.get_columns();
        if table_schema.iter().zip(child_schema).any(|(col1, col2)| col1.get_type() != col2.get_type()) {
            panic!("table schema mismatch");
        }

        let has_returning = !self.get_returning().is_empty();

        let insert_schema = if has_returning {
            // TODO - fix this, we should not do this!
            Arc::new(self.get_table().schema.prefix_column_names(self.get_table().table.as_str()))
        } else {
            Arc::new(Schema::new(vec![
                // TODO - use this
                Column::new_fixed_size("__bustub_internal.insert_rows".to_string(), DBTypeId::INT)
            ]))
        };

        let mut plan = InsertPlan::new(
            insert_schema,
            select,
            self.get_table().oid,
        ).into();
        
        if has_returning {
            let select_list_children = vec![&plan];
            
            let (column_names, exprs): (Vec<String>, Vec<ExpressionRef>) = self.get_returning()
                .iter()
                .map(|item| item.plan(select_list_children.as_slice(), planner))
                .unzip();

            plan = ProjectionPlanNode::new(
                Arc::new(ProjectionPlanNode::rename_schema(
                    ProjectionPlanNode::infer_projection_schema(exprs.as_slice()),
                    column_names.as_slice(),
                )),
                exprs,
                plan,
            ).into()
        }

        plan
    }
}
