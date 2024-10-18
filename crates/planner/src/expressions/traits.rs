use crate::plan_nodes::PlanType;
use crate::Planner;
use std::fmt::Debug;
use std::rc::Rc;
use expression::{ExpressionRef, ExpressionType};

/// plan table ref
pub(crate) trait PlanExpression {
    fn plan<'a>(&self, children: &[&PlanType], planner: &'a Planner<'a>) -> (String, ExpressionRef);
}
