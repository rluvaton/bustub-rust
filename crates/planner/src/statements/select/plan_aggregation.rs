use crate::expressions::PlanExpression;
use crate::statements::select::plan_normal_select::PlanNormalSelect;
use crate::{AggregationPlanNode, AggregationType, FilterPlan, PlanNode, PlanType, Planner, ProjectionPlanNode};
use binder::{AggCallExpr, ExpressionTypeImpl, SelectStatement};
use data_types::{DBTypeId, Value};
use expression::{ColumnValueExpression, ConstantValueExpression, Expression, ExpressionRef};
use std::rc::Rc;
use std::sync::Arc;

pub(crate) trait PlanAggregation {
    fn plan_aggregation(&self, child: PlanType, planner: &Planner) -> PlanType;
}

impl PlanAggregation for SelectStatement {
    fn plan_aggregation(&self, child: PlanType, planner: &Planner) -> PlanType {
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

        let children = vec![&child];


        // Create a new context which allows aggregation call.
        let guard = planner.new_context();
        guard.context.lock().allow_aggregation = true;

        // Plan group by expressions

        let mut group_by_exprs: Vec<ExpressionRef> = vec![];
        let mut output_col_names: Vec<String> = vec![];

        for expr in &self.group_by {
            let (col_name, abstract_expr) = expr.plan(children.as_slice(), planner);

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
        let mut input_exprs = vec![];
        let mut agg_types = vec![];

        // agg-calls will be after group-bys in the output of agg.
        let agg_begin_idx = group_by_exprs.len();

        let mut term_idx = 0;

        // TODO - Can this be a problem
        let aggregations = planner.context.lock().aggregations.clone();

        for item in aggregations {
            let agg_call = match &*item {
                ExpressionTypeImpl::AggCall(agg_call) => agg_call,
                _ => unreachable!()
            };

            let (agg_type, exprs) = plan_agg_call(agg_call, children.as_slice(), planner);

            input_exprs.push(match exprs.len() {
                // Rewrite count(*) into count(1)
                0 => ConstantValueExpression::new(Value::from(1)).into_ref(),
                1 => exprs[0].clone(),
                _ => unimplemented!("only agg call of zero/one arg is supported"),
            });

            agg_types.push(agg_type);
            output_col_names.push(format!("agg#{}", term_idx));
            planner.context.lock().expr_in_agg.push(
                ColumnValueExpression::new(0, agg_begin_idx + term_idx, DBTypeId::INT).into_ref()
            );

            term_idx += 1;
        }


        let agg_output_schema = AggregationPlanNode::infer_agg_schema(group_by_exprs.as_slice(), input_exprs.as_slice(), agg_types.as_slice());

        // Create the aggregation plan node for the first phase (finally!)
        let mut plan: PlanType = AggregationPlanNode::new(
            Arc::new(ProjectionPlanNode::rename_schema(agg_output_schema, output_col_names.as_slice())),
            child,
            group_by_exprs,
            input_exprs,
            agg_types
        ).into();

        // Phase-2: plan filter / projection to match the original select list

        // Create filter based on the having clause
        if let Some(having) = &self.having {
            let children = vec![&plan];
            let (_, expr) = having.plan(children.as_slice(), planner);
            plan = FilterPlan::new(
                plan.get_output_schema(),
                expr,
                plan
            ).into();
        }

        // Plan normal select (within aggregation context)

        self.plan_normal_select(plan, planner)
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
            planner.context.lock().add_aggregations(Rc::new(agg.clone().into()))
        },
        _ => panic!("expression type {:?} not supported in planner yet", expr)
    }
}

fn plan_agg_call(agg_call: &AggCallExpr, children: &[&PlanType], planner: &Planner) -> (AggregationType, Vec<ExpressionRef>) {
    if agg_call.is_distinct {
        unimplemented!("distinct agg is not implemented yet");
    }

    let exprs = {
        // Create a new context that doesn't allow aggregation calls.
        let _guard = planner.new_context();

        agg_call.args
            .iter()
            .map(|arg| arg.plan(children, planner).1)
            .collect::<Vec<_>>()
    };

    get_agg_call(agg_call.func.as_str(), exprs)
}

fn get_agg_call(func_name: &str, args: Vec<ExpressionRef>) -> (AggregationType, Vec<ExpressionRef>) {
    if args.is_empty() {
        return match func_name {
            "count_str" => (AggregationType::CountStarAggregate, vec![]),
            _ => panic!("unsupported agg_call {} with {} args", func_name, args.len())
        }
    }

    assert_eq!(args.len(), 1, "Must only have 1 argument - unsupported agg_call {} with {} args", func_name, args.len());

    match func_name {
        "min" => (AggregationType::MinAggregate, args),
        "max" => (AggregationType::MaxAggregate, args),
        "sum" => (AggregationType::SumAggregate, args),
        "count" => (AggregationType::CountAggregate, args),
        _ => panic!("unsupported agg_call {} with {} args", func_name, args.len())
    }
}
