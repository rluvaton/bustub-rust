use crate::expressions::PlanExpression;
use crate::{PlanType, Planner};
use binder::{AggCallExpr, ExpressionTypeImpl, SelectStatement};
use expression::ExpressionRef;
use std::rc::Rc;

pub(crate) trait PlanAggregation {
    fn plan_aggregation(&self, child: &PlanType, planner: &Planner) -> PlanType;
}

impl PlanAggregation for SelectStatement {
    fn plan_aggregation(&self, child: &PlanType, planner: &Planner) -> PlanType {
        /* Transforming hash agg is complex. Let's see a concrete example here.
    *
    * Now that we have,
    * ```
    * select v3, max(v1) + max(v2) from table where <cond> group by v3 having count(v4) > count(v5);
    * ```
    *
    * Before this comment section, we have `from table where <cond>` planned as filter / table scan.
    *
    * Given this is a group-by query, we will have something like this in the final plan,
    * ```
    * <Parent Plan Nodes>
    *   Projection v3, max(v1) + max(v2)
    *     Filter count(v4) > count(v5)
    *       Aggregation group_by=v3 agg_types=[max, max, count, count] agg_expr=[v1, v2, v4, v5]
    *         <Filter / Table Scan> --- "table scan child"
    * ```
    *
    * For every expression in select list and having clause, we will do a two-phase planning. Firstly,
    * we plan all aggregation calls, and then we plan other expressions. Let's take `max(v1) + max(v2)` as
    * an example.
    *
    * - We plan `max(v1)` and `max(v2)` in the select list and `count(v4)`, `count(v5)` using the
    *   "table scan child". We get tuples of `[agg_type, abstract_expression]`.
    * - Then we construct the aggregation plan node with all required aggregations plus group-by expressions.
    * - After that we plan the filter `count(v4) > count(v5)` and the projection `v3`, `max(v1) + max(v2)`.
    * - That's all!
    */


        // Create a new context which allows aggregation call.
        let guard = planner.new_context();
        guard.context.lock().allow_aggregation = true;

        // Plan group by expressions

        let mut group_by_exprs: Vec<ExpressionRef> = vec![];
        let mut output_col_names: Vec<String> = vec![];

        for expr in &self.group_by {
            let (col_name, abstract_expr) = expr.plan(&vec![child], planner);

            output_col_names.push(col_name);
            group_by_exprs.push(abstract_expr);
        }

        // Rewrite all agg call inside having.
        if let Some(having) = &self.having {
            add_agg_call_to_context(having, planner);
        }


        // Rewrite all agg call inside expression to a pseudo one.
        // It replaces the agg call in select_list_ with a pseudo one with index
        // adds the real agg call to context.
        for item in &self.select_list {
            add_agg_call_to_context(item, planner);
        }

        // Phase-1: plan an aggregation plan node out of all of the information we have.
        // let input_exprs = vec![];

        todo!();
    }
}


fn add_agg_call_to_context(expr: &binder::ExpressionTypeImpl, planner: &Planner) {
    match expr {
        ExpressionTypeImpl::ColumnRef(_) | ExpressionTypeImpl::Constant(_) => return,
        ExpressionTypeImpl::Alias(alias) => return add_agg_call_to_context(&*alias.child, planner),
        ExpressionTypeImpl::BinaryOp(expr) => {
            add_agg_call_to_context(&*expr.larg, planner);
            add_agg_call_to_context(&*expr.rarg, planner);

            return;
        }
        ExpressionTypeImpl::FuncCall(f) => {
            for child in &f.args {
                add_agg_call_to_context(&*child, planner);
            }

            return;
        }
        ExpressionTypeImpl::AggCall(agg) => {
            let agg_name = format!("__pseudo_agg#{}", planner.context.lock().aggregations.len());
            let agg_call = AggCallExpr::new(agg_name, vec![], agg.is_distinct);

            // Replace the agg call in the original bound expression with a pseudo one, add agg call to the context.
            // TODO - in the original code there was std::exchange(agg_call_expr, std::move(agg_call))));
            planner.context.lock().add_aggregations(Rc::new(agg_call.into()))
        },
        _ => panic!("expression type {:?} not supported in planner yet", expr)
    }
}
