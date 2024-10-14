use std::sync::Arc;
use crate::expressions::{Expression, ExpressionTypeImpl};

/// All types of order-bys in binder.

#[derive(Debug, PartialEq)]
pub(crate) enum OrderByType {
    Invalid,
    Default,
    Asc,
    Desc
}

impl Default for OrderByType {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct OrderBy {
    order_type: OrderByType,
    expr: Arc<ExpressionTypeImpl>
}
