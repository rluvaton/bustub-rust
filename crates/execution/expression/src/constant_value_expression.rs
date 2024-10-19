use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use tuple::Tuple;
use crate::expression_type::ExpressionType;
use crate::traits::{Expression, ExpressionRef, NO_CHILDREN};


/// ConstantValueExpression represents constants.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstantValueExpression(Value);

impl ConstantValueExpression {

    /** Creates a new constant value expression wrapping the given value. */

    pub fn new(value: Value) -> Self {
        Self(value)
    }
}


impl Display for ConstantValueExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<ExpressionType> for ConstantValueExpression {
    fn into(self) -> ExpressionType {
        ExpressionType::Constant(self)
    }
}

impl Expression for ConstantValueExpression {
    fn evaluate(&self, _tuple: &Tuple, _schema: &Schema) -> Value {
        self.0.clone()
    }

    fn evaluate_join(&self, _left_tuple: &Tuple, _left_schema: &Schema, _right_tuple: &Tuple, _right_schema: &Schema) -> Value {
        self.0.clone()
    }

    fn get_children(&self) -> &[ExpressionRef] {
        NO_CHILDREN
    }

    fn get_return_type(&self) -> DBTypeId {
        self.0.get_db_type_id()
    }
}
