use crate::expressions::{Expression, ExpressionType, ExpressionTypeImpl};

/// A bound column reference, e.g., `y.x` in the SELECT list.
#[derive(Debug, PartialEq)]
pub(crate) struct ColumnRef {
    pub(crate) col_name: Vec<String>
}

impl ColumnRef {

    pub(crate) fn new(col_name: Vec<String>) -> Self {
        Self {
            col_name
        }
    }

    pub fn prepend(&mut self, prefix: String) -> Self {
        unimplemented!()
    }
}

impl Into<ExpressionTypeImpl> for ColumnRef {
    fn into(self) -> ExpressionTypeImpl {
        ExpressionTypeImpl::ColumnRef(self)
    }
}

impl Expression for ColumnRef {
    fn has_aggregation(&self) -> bool {
        false
    }
}
