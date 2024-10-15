use std::rc::Rc;
use std::sync::Arc;
use crate::plan_nodes::{FilterPlan, PlanNode, PlanNodeRef, PlanType, ValuesPlanNode};
use crate::traits::Plan;
use crate::Planner;
use binder::{Expression, SelectStatement, TableReferenceTypeImpl};
use catalog_schema::Schema;
use crate::expressions::PlanExpression;
use crate::statements::select::plan_window::PlanWindow;

impl Plan for SelectStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> Rc<PlanType> {
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

            let (_, expr) = where_expr.plan(vec![plan.clone()], planner);
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
        }

        todo!()
    }
}
