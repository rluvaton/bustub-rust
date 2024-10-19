use crate::constants::UNNAMED_COLUMN;
use crate::expressions::traits::PlanExpression;
use crate::plan_nodes::PlanType;
use crate::Planner;
use binder::Constant;
use expression::{ConstantValueExpression, Expression, ExpressionRef};

impl PlanExpression for Constant {
    fn plan<'a>(&self, _children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        (UNNAMED_COLUMN.to_string(), ConstantValueExpression::new(self.value.clone()).into_ref())
    }
}
