use std::sync::Arc;
use binder::SelectStatement;
use expression::ExpressionRef;
use crate::{PlanType, Planner, ProjectionPlanNode};
use crate::expressions::PlanExpression;

pub(crate) trait PlanNormalSelect {
    fn plan_normal_select(&self, child: PlanType, planner: &Planner) -> PlanType;
}

impl PlanNormalSelect for SelectStatement {
    fn plan_normal_select(&self, child: PlanType, planner: &Planner) -> PlanType {
        let select_list_children = vec![&child];
        // Plan normal select
        let (column_names, exprs): (Vec<String>, Vec<ExpressionRef>) = self.select_list
            .iter()
            .map(|item| item.plan(select_list_children.as_slice(), planner))
            .unzip();

        ProjectionPlanNode::new(
            Arc::new(ProjectionPlanNode::rename_schema(
                ProjectionPlanNode::infer_projection_schema(exprs.as_slice()),
                column_names.as_slice(),
            )),
            exprs,
            child,
        ).into()
    }
}
