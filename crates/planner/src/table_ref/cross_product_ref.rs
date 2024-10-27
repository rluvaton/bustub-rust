use std::rc::Rc;
use std::sync::Arc;
use binder::{CrossProductRef, JoinType};
use data_types::Value;
use expression::{ConstantValueExpression, Expression};
use crate::plan_nodes::PlanType;
use crate::{NestedLoopJoinPlanNode, Planner};
use crate::traits::Plan;

impl Plan for CrossProductRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> error_utils::anyhow::Result<PlanType> {
        let left = self.left.plan(planner)?;
        let right = self.right.plan(planner)?;

        Ok(NestedLoopJoinPlanNode::new(
            Arc::new(NestedLoopJoinPlanNode::infer_join_schema(&left, &right)),
            left,
            right,
            ConstantValueExpression::new(Value::from(true)).into_ref(),
            JoinType::Inner
        ).into())
    }
}
