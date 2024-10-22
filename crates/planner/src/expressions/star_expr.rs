use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::StarExpr;
use data_types::Value;
use expression::{ConstantValueExpression, Expression, ExpressionRef};
use crate::constants::UNNAMED_COLUMN;

impl PlanExpression for StarExpr {
    fn plan<'a>(&self, _children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        (UNNAMED_COLUMN.to_string(), ConstantValueExpression::new(Value::from(0)).into_ref())
    }
}
