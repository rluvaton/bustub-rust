use std::rc::Rc;
use std::sync::Arc;
use binder::{Expression as BinderExpression, ExpressionTypeImpl, OrderByType, SelectStatement, WindowBoundary};
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use expression::{ColumnValueExpression, ConstantValueExpression, Expression, ExpressionRef};
use crate::constants::UNNAMED_COLUMN;
use crate::expressions::PlanExpression;
use crate::plan_nodes::{PlanNode, PlanType, ProjectionPlanNode, WindowFunctionPlanNode, WindowFunctionType};
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
        let mut window_func_types = vec![];
        let mut partition_by_exprs = vec![];
        let mut order_by_exprs = vec![];
        let mut arg_exprs = vec![];

        for (index, item) in self.select_list.iter().enumerate() {
            if !item.has_window_function() {
                // Normal select
                let (mut name, expr) = item.plan(vec![child.clone()], planner);
                columns.push(expr);

                if name == UNNAMED_COLUMN {
                    name = format!("__unnamed#{}", planner.get_and_increment_universal_id());
                }

                column_names.push(name);

                continue;
            }

            // parse window function
            window_func_indexes.push(index);

            // We assign a -1 here as a placeholder
            columns.push(ColumnValueExpression::new(0, usize::MAX, DBTypeId::INT).into_ref());

            let window_call = match item {
                ExpressionTypeImpl::Alias(alias) => {
                    column_names.push(alias.alias.clone());
                    match &*alias.child {
                        ExpressionTypeImpl::Window(window) => window,
                        _ => panic!("Invalid alias to non window function"),
                    }
                }
                ExpressionTypeImpl::Window(window) => {
                    column_names.push(format!("__unnamed#{}", planner.get_and_increment_universal_id()));

                    window
                }
                _ => panic!("Invalid expression type {:?} has window function", item)
            };

            assert_ne!(window_call.start, Some(WindowBoundary::UnboundedPreceding), "Bustub currently only support window function with default window frame settings");
            assert!(window_call.end == Some(WindowBoundary::CurrentRowRows) || window_call.end == Some(WindowBoundary::CurrentRowRange), "Bustub currently only support window function with default window frame settings, window end is {:?}", window_call.end);

            partition_by_exprs.push(
                window_call.partition_by
                    .iter()
                    .map(|item| item.plan(vec![child.clone()], planner).1)
                    .collect::<Vec<_>>()
            );

            assert!(window_call.func != "rank" || !window_call.order_bys.is_empty(), "order by clause is mandatory for rank function");

            order_by_exprs.push(
                window_call.order_bys
                    .iter()
                    .map(|item| (item.order_type, item.expr.plan(vec![child.clone()], planner).1))
                    .collect::<Vec<(OrderByType, ExpressionRef)>>()
            );

            let raw_args = window_call.args.iter().map(|item| item.plan(vec![child.clone()], planner).1).collect::<Vec<_>>();

            let (window_func_type, clean_args) = get_window_agg_call(window_call.func.as_str(), raw_args);
            window_func_types.push(window_func_type);
            assert!(clean_args.len() <= 1, "only agg call of zero/one arg is supported");

            let clean_arg = match clean_args.len() {
                0 => ConstantValueExpression::new(Value::from(1)).into_ref(),
                1 => clean_args[0].clone(),
                _ => panic!("only agg call of zero/one arg is supported")
            };

            arg_exprs.push(clean_arg);
        }

        assert_order_bys_are_compatible(order_by_exprs.as_slice());

        // we don't need window_agg_indexes here because we already use placeholders to infer the window agg column type is
        // Integer
        let window_output_schema = WindowFunctionPlanNode::infer_window_schema(columns.as_slice());

        WindowFunctionPlanNode::new(
            Arc::new(ProjectionPlanNode::rename_schema(window_output_schema, column_names.as_slice())),
            child,
            window_func_indexes,
            columns,
            partition_by_exprs,
            order_by_exprs,
            arg_exprs,
            window_func_types
        ).into_ref()
    }
}


fn get_window_agg_call(func_name: &str, args: Vec<ExpressionRef>) -> (WindowFunctionType, Vec<ExpressionRef>) {
    if args.is_empty() {
        return match func_name {
            "count_str" => (WindowFunctionType::CountStarAggregate, vec![]),
            "rank" => (WindowFunctionType::Rank, vec![]),
            _ => panic!("unsupported window_call {} with {} args", func_name, args.len())
        }
    }

    assert_eq!(args.len(), 1, "Must only have 1 argument - unsupported window_call {} with {} args", func_name, args.len());

    match func_name {
        "min" => (WindowFunctionType::MinAggregate, args),
        "max" => (WindowFunctionType::MaxAggregate, args),
        "sum" => (WindowFunctionType::SumAggregate, args),
        "count" => (WindowFunctionType::CountAggregate, args),
        _ => panic!("unsupported window_call {} with {} args", func_name, args.len())
    }
}

fn assert_order_bys_are_compatible(order_by_exprs:&[Vec<(OrderByType, ExpressionRef)>]) {
    if order_by_exprs.is_empty() {
        // either or window functions not having order by clause
        return;
    }

    // or all order by clause are the same
    let first_order_by = &order_by_exprs[0];

    for order_by in order_by_exprs {
        assert_eq!(order_by.len(), first_order_by.len(), "order by clause of window functions are not compatible");

        for (index, item) in order_by.iter().enumerate() {
            assert_eq!(item.0, first_order_by[index].0, "order by clause of window functions are not compatible");
            assert_eq!(item.1.to_string(), first_order_by[index].1.to_string(), "order by clause of window functions are not compatible");
        }
    }
}
