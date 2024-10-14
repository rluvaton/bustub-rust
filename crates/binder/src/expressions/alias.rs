use std::sync::Arc;
use crate::expressions::{ColumnRef, Expression, ExpressionType, ExpressionTypeImpl};

/// The alias in SELECT list, e.g. `SELECT count(x) AS y`, the `y` is an alias.
#[derive(Debug, PartialEq)]
pub(crate) struct Alias {

    /// The alias
    pub(crate) alias: String,

    /// The actual expression
    pub(crate) child: Arc<ExpressionTypeImpl>
}

impl Alias {

    pub(crate) fn new(alias: String, child: Arc<ExpressionTypeImpl>) -> Self {
        Self {
            alias,
            child
        }
    }
}

impl Into<ExpressionTypeImpl> for Alias {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::Alias(self)
    }
}

impl Expression for Alias {

    fn has_aggregation(&self) -> bool {
        self.child.has_aggregation()
    }

    fn has_window_function(&self) -> bool {
        self.child.has_window_function()
    }
}
