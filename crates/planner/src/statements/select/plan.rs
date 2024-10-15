use std::rc::Rc;
use std::sync::Arc;
use crate::plan_nodes::{FilterPlan, PlanNode, PlanType, ValuesPlanNode};
use crate::traits::Plan;
use crate::Planner;
use binder::{SelectStatement, TableReferenceTypeImpl};
use catalog_schema::Schema;
use crate::expressions::PlanExpression;

impl Plan for SelectStatement {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> Rc<PlanType> {
        let ctx_guard = planner.new_context();

        if !self.ctes.is_empty() {
            ctx_guard.context.lock().cte_list.replace(self.ctes.clone());
        }

        let mut plan: Rc<PlanType> = match &*self.table {
            TableReferenceTypeImpl::Empty => {
                ValuesPlanNode::new(Arc::new(Schema::new(vec![])), vec![vec![]]).into_rc_plan_type()
            }
            _ => {
                self.table.plan(planner)
            }
        };

        if let Some(where_expr) = &self.where_exp {
            let schema = plan.get_output_schema();

            let (_, expr) = where_expr.plan(vec![plan.clone()], planner);
            plan = FilterPlan::new(schema, expr, plan).into_rc_plan_type();
        }

        todo!()
    }
}
