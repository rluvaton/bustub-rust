use crate::plan_nodes::window_plan_node::window_function_type::WindowFunctionType;
use crate::plan_nodes::PlanNodeRef;
use binder::OrderByType;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct WindowFunction {
    pub function: PlanNodeRef,
    pub fn_type: WindowFunctionType,
    pub partition_by: Vec<PlanNodeRef>,
    pub order_by: Vec<(OrderByType, PlanNodeRef)>,
}

impl Display for WindowFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowFunction")
            .field("function_arg", &self.function)
            .field("type", &self.fn_type)
            .field("partition_by", &self.partition_by)
            .field("order_by", &self.order_by)
            .finish()
    }
}
