use crate::expressions::{Expression, ExpressionType};

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

impl Expression for ColumnRef {
    const TYPE: ExpressionType = ExpressionType::ColumnRef;

    fn get_type(&self) -> ExpressionType {
        Self::TYPE
    }

    fn has_aggregation(&self) -> bool {
        false
    }
}
