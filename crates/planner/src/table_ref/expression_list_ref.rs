use std::rc::Rc;
use std::sync::Arc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::{Planner, ValuesPlanNode};
use binder::ExpressionListRef;
use catalog_schema::{Column, Schema};
use common::config::VARCHAR_DEFAULT_LENGTH;
use data_types::DBTypeId;
use expression::Expression;
use crate::expressions::PlanExpression;

impl Plan for ExpressionListRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        let all_exprs = self.values
            .iter()
            .map(|row| {
                row
                    .iter()
                    .map(|col| col.plan(&[], planner).1)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<_>>>();

        let first_row = &all_exprs[0];

        let schema: Schema = first_row
            .iter()
            .enumerate()
            .map(|(index, col)| {
                let col_name = format!("{}.{}", self.identifier, index);

                match col.get_return_type() {
                    DBTypeId::VARCHAR => Column::new_variable_size(col_name, col.get_return_type(), VARCHAR_DEFAULT_LENGTH as u32),
                    _ => Column::new_fixed_size(col_name, col.get_return_type()),
                }
            })
            .into();

        ValuesPlanNode::new(Arc::new(schema), all_exprs).into()
    }
}
