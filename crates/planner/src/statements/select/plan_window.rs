use std::rc::Rc;
use binder::{Expression as BinderExpression, SelectStatement};
use data_types::DBTypeId;
use expression::{ColumnValueExpression, Expression};
use crate::expressions::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;

pub(crate) trait PlanWindow {
    fn plan_window(&self, child: Rc<PlanType>, planner: &Planner) -> Rc<PlanType>;
}

impl PlanWindow for SelectStatement {
    fn plan_window(&self, child: Rc<PlanType>, planner: &Planner) -> Rc<PlanType> {
        /* For window function we don't do two passes rewrites like planning normal aggregations.
   *  Because standard sql does not allow using window function results in where clause, and
   *  our implementation does not support having on window function. We assume window functions
   *  only appear in select list, and we can plan them in one pass.
   */

        let mut columns = vec![];
        let mut column_names = vec![];
        let mut window_func_indexes = vec![];
        let mut partition_by_exprs = vec![];
        let mut order_by_exprs = vec![];
        let mut arg_exprs = vec![];

        for (index, item) in self.select_list.iter().enumerate() {
            if !item.has_window_function() {
                // Normal select
                let (name, expr) = item.plan(vec![child], planner);
                columns.push(expr);

                column_names.push(
                    name.unwrap_or_else(|| format!("__unnamed#{}", planner.get_and_increment_universal_id()))
                );

                continue;
            }

            // parse window function
            window_func_indexes.push(index);

            // We assign a -1 here as a placeholder
            columns.push(ColumnValueExpression::new(0, usize::MAX, DBTypeId::INT).into_ref())
        }

        todo!()
    }
}
