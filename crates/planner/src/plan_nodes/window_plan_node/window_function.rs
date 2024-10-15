use binder::OrderByType;
use expression::ExpressionRef;
use std::fmt::{Display, Formatter};
use crate::plan_nodes::WindowFunctionType;

#[derive(Clone, Debug, PartialEq)]
pub struct WindowFunction {
    pub function: ExpressionRef,
    pub fn_type: WindowFunctionType,
    pub partition_by: Vec<ExpressionRef>,
    pub order_by: Vec<(OrderByType, ExpressionRef)>,
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
