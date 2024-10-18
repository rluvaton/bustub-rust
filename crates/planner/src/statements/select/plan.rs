use crate::expressions::PlanExpression;
use crate::plan_nodes::{AggregationPlanNode, FilterPlan, PlanNode, PlanNodeRef, ProjectionPlanNode, ValuesPlanNode};
use crate::statements::select::plan_aggregation::PlanAggregation;
use crate::statements::select::plan_window::PlanWindow;
use crate::traits::Plan;
use crate::{LimitPlanNode, Planner};
use binder::{Expression as BinderExpression, ExpressionTypeImpl, SelectStatement, TableReferenceTypeImpl};
use catalog_schema::Schema;
use expression::{ColumnValueExpression, Expression, ExpressionRef};
use std::sync::Arc;

impl Plan for SelectStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanNodeRef {
        let ctx_guard = planner.new_context();

        if !self.ctes.is_empty() {
            ctx_guard.context.lock().cte_list.replace(self.ctes.clone());
        }

        let mut plan: PlanNodeRef = match &*self.table {
            TableReferenceTypeImpl::Empty => {
                ValuesPlanNode::new(Arc::new(Schema::new(vec![])), vec![vec![]]).into_ref()
            }
            _ => {
                self.table.plan(planner)
            }
        };

        if let Some(where_expr) = &self.where_exp {
            let schema = plan.get_output_schema();

            let (_, expr) = where_expr.plan(&vec![plan.clone()], planner);
            plan = FilterPlan::new(schema, expr, plan).into_ref();
        }

        // Binder already checked that normal aggregations and window aggregations cannot coexist.
        let (has_agg, has_window_agg) = {
            let expr = self.select_list.iter().find(|e| e.has_aggregation() || e.has_window_function());

            (
                expr.map(|e| e.has_aggregation()).unwrap_or(false),
                expr.map(|e| e.has_window_function()).unwrap_or(false),
            )
        };

        if has_window_agg {
            assert_eq!(self.having, None, "HAVING on window function is not supported yet.");
            assert_eq!(self.group_by.is_empty(), true, "Group by is not allowed to use with window function.");

            plan = self.plan_window(plan, planner);
        } else if self.having.is_some() || !self.group_by.is_empty() || has_agg {
            // Plan aggregation
            plan = self.plan_aggregation(plan, planner);
        } else {
            // Plan normal select
            let (column_names, exprs): (Vec<String>, Vec<ExpressionRef>) = self.select_list
                .iter()
                .map(|item| item.plan(&vec![plan.clone()], planner))
                .unzip();

            plan = ProjectionPlanNode::new(
                Arc::new(ProjectionPlanNode::rename_schema(
                    ProjectionPlanNode::infer_projection_schema(exprs.as_slice()),
                    column_names.as_slice()
                )),
                exprs,
                plan
            ).into_ref();
        }

        // Plan DISTINCT as group agg

        if self.is_distinct {
            let distinct_exprs = plan
                .get_output_schema()
                .get_columns()
                .iter()
                .enumerate()
                .map(|(index, col)| ColumnValueExpression::new(0, index, col.get_type()).into_ref())
                .collect::<Vec<_>>();

            plan = AggregationPlanNode::new(
                plan.get_output_schema(),
                plan,
                distinct_exprs,
                vec![],
                vec![],
            ).into_ref();
        }

        // Plan ORDER BY
        if !self.sort.is_empty() {
            unimplemented!()
        }

        // Plan LIMIT and OFFSET
        if let Some(limit) = &self.limit_count {
            let e = match limit {
                ExpressionTypeImpl::Constant(e) => e,
                _ => unimplemented!("Currently only constant integer as an offset is supported")
            };

            let limit_count: Option<i64>  = e.value.clone().try_into().expect("Limit constant must be a number");
            let limit_count = limit_count.expect("Limit constant must not be null");

            assert!(limit_count >= 0, "Limit count cant be negative");

            plan = LimitPlanNode::new(plan.get_output_schema(), plan, limit_count as usize).into_ref();
        }

        if self.limit_offset.is_some() {
            unimplemented!()
        }


        plan
    }
}
