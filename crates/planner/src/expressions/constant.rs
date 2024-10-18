use std::rc::Rc;
use crate::plan_nodes::PlanType;
use crate::traits::Plan;
use crate::Planner;
use binder::{ColumnRef, Constant};
use expression::{ConstantValueExpression, Expression, ExpressionRef};
use crate::constants::UNNAMED_COLUMN;
use crate::expressions::traits::PlanExpression;

impl PlanExpression for Constant {
    fn plan<'a>(&self, _children: &[&PlanType], _planner: &'a Planner<'a>) -> (String, ExpressionRef) {
        (UNNAMED_COLUMN.to_string(), ConstantValueExpression::new(self.value.clone()).into_ref())
    }
}
