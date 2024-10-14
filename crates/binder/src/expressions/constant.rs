use data_types::Value;
use crate::expressions::{Expression, ExpressionType, ExpressionTypeImpl};

/// A bound constant, e.g., `1`.
#[derive(Debug, PartialEq)]
pub(crate) struct Constant {
    pub(crate) value: Value
}

impl Constant {

    pub(crate) fn new(value: Value) -> Self {
        Self {
            value
        }
    }
}


impl Into<ExpressionTypeImpl> for Constant {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Constant(self)
    }
}

impl Expression for Constant {
    fn has_aggregation(&self) -> bool {
        false
    }
}
