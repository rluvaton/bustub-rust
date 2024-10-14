use std::sync::Arc;
use crate::expressions::{Expression, ExpressionType, ExpressionTypeImpl};

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

impl Expression for Alias {
    const TYPE: ExpressionType = ExpressionType::ColumnRef;

    fn get_type(&self) -> ExpressionType {
        Self::TYPE
    }

    fn has_aggregation(&self) -> bool {
        self.child.has_aggregation()
    }

    fn has_window_function(&self) -> bool {
        self.child.has_window_function()
    }
}
