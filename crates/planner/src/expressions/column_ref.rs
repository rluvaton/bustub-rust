use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::{PlanNode, PlanType};
use crate::Planner;
use binder::ColumnRef;
use expression::{ColumnValueExpression, Expression, ExpressionRef};

impl PlanExpression for ColumnRef {
    fn plan<'a>(&self, children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        let col_name = self.to_string();

        match children.len() {
            0 => panic!("column ref should have at least one child"),
            1 => {
                // Projections, Filters, and other executors evaluating expressions with one single child will
                // use this branch.
                let child = children[0];
                let schema = child.get_output_schema();

                // Before we can call `schema.GetColIdx`,  we need to ensure there's no duplicated column.
                if schema.get_columns().iter().filter(|col| col.get_name() == &col_name).count() > 1 {
                    panic!("duplicated column found in schema");
                }

                let col_idx = schema.get_col_idx(col_name.as_str());
                let col_type = schema.get_column(col_idx).get_type();

                (col_name, ColumnValueExpression::new(0, col_idx, col_type).into_ref())
            }
            2 => {
                /*
                 * Joins will use this branch to plan expressions.
                 *
                 * If an expression is for join condition, e.g.
                 * SELECT * from test_1 inner join test_2 on test_1.colA = test_2.col2
                 * The plan will be like:
                 * ```
                 * NestedLoopJoin condition={ ColumnRef 0.0=ColumnRef 1.1 }
                 *   SeqScan colA, colB
                 *   SeqScan col1, col2
                 * ```
                 * In `ColumnRef n.m`, when executor is using the expression, it picks from its
                 * nth children's mth column to get the data.
                 */


                let left = &children[0];
                let right = &children[1];
                let left_schema =  left .get_output_schema();
                let right_schema = right.get_output_schema();

                let col_idx_left = left_schema.try_get_col_idx(col_name.as_str());
                let col_idx_right = right_schema.try_get_col_idx(col_name.as_str());
                if col_idx_left.is_some() && col_idx_right.is_some() {
                    panic!("ambiguous column name {}", col_name);
                }

                if let Some(col_idx_left) = col_idx_left {
                    let col_type = left_schema.get_column(col_idx_left).get_type();
                    (col_name, ColumnValueExpression::new(0, col_idx_left, col_type).into_ref())
                } else if let Some(col_idx_right) = col_idx_right {
                    let col_type = left_schema.get_column(col_idx_right).get_type();
                    (col_name, ColumnValueExpression::new(0, col_idx_right, col_type).into_ref())
                } else {
                    panic!("column name {} not found", col_name);
                }
            }
            _ => unimplemented!("no executor with expression has more than 2 children for now")
        }
    }
}
