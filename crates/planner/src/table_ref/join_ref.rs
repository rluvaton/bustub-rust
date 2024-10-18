use std::sync::Arc;
use crate::plan_nodes::PlanType;
use crate::{NestedLoopJoinPlanNode, Planner};
use binder::{JoinRef, JoinType};
use data_types::Value;
use expression::{ConstantValueExpression, Expression};
use crate::expressions::PlanExpression;
use crate::traits::Plan;

impl Plan for JoinRef {
    fn plan<'a>(&self, planner: &'a Planner<'a>)-> PlanType {
        let left = self.left.plan(planner);
        let right = self.right.plan(planner);
        let children = vec![&left, &right];

        let join_condition = if let Some(condition) = &self.condition {
            condition.plan(children.as_slice(), planner).1
        } else {
            ConstantValueExpression::new(Value::from(true)).into_ref()
        };

        NestedLoopJoinPlanNode::new(
            Arc::new(NestedLoopJoinPlanNode::infer_join_schema(&left, &right)),
            left,
            right,
            join_condition,
            JoinType::Inner
        ).into()
    }
}
