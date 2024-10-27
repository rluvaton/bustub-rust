use std::sync::Arc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::{PlanNode, Planner, ProjectionPlanNode};
use binder::SubqueryRef;
use expression::{ColumnValueExpression, Expression, ExpressionRef};

impl Plan for SubqueryRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> error_utils::anyhow::Result<PlanType> {
        let select = self.subquery.plan(planner)?;

        // This projection will be removed by eliminate projection rule. It's solely used for renaming columns.
        let (column_names, exprs): (Vec<String>, Vec<ExpressionRef>) = select.get_output_schema().get_columns()
            .iter()
            .enumerate()
            .map(|(index, col)| (
                format!("{}.{}", self.alias, self.select_list_name[index].join(".")),
                ColumnValueExpression::new(0, index, col.get_type()).into_ref()
            ))
            .unzip();

        Ok(ProjectionPlanNode::new(
            Arc::new(ProjectionPlanNode::rename_schema(
                ProjectionPlanNode::infer_projection_schema(exprs.as_slice()),
                column_names.as_slice(),
            )),
            exprs,
            select,
        ).into())
    }
}
