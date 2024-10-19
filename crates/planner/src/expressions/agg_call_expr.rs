use crate::constants::UNNAMED_COLUMN;
use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::AggCallExpr;
use expression::ExpressionRef;

impl PlanExpression for AggCallExpr {
    fn plan<'a>(&self, _children: &[&PlanType], planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        let mut ctx = planner.context.lock();

        assert!(ctx.next_aggregation < ctx.expr_in_agg.len(), "unexpected agg call");

        let i = ctx.next_aggregation;

        ctx.next_aggregation += 1;

        (UNNAMED_COLUMN.to_string(), ctx.expr_in_agg[i].clone())
    }
}
