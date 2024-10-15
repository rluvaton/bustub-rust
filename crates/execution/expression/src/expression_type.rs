use crate::traits::{Expression, ExpressionRef};
use crate::{ArithmeticExpression, ColumnValueExpression, ComparisonExpression, ConstantValueExpression, LogicExpression, StringExpression};
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use std::fmt::{Display, Formatter};
use tuple::Tuple;

// Helper to avoid duplicating deref on each variant
macro_rules! call_each_variant {
    ($enum_val:expr, $name:ident, $func:expr) => {
        match $enum_val {
            ExpressionType::ColumnValue($name) => $func,
            ExpressionType::Arithmetic($name) => $func,
            ExpressionType::Comparison($name) => $func,
            ExpressionType::Constant($name) => $func,
            ExpressionType::Logic($name) => $func,
            ExpressionType::String($name) => $func,
            // Add match arms for other variants as necessary
        }
    };
}


#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionType {
    ColumnValue(ColumnValueExpression),
    Arithmetic(ArithmeticExpression),
    Comparison(ComparisonExpression),
    Constant(ConstantValueExpression),
    Logic(LogicExpression),
    String(StringExpression),
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        call_each_variant!(self, p, {
            p.fmt(f)
        })
    }
}

impl Expression for ExpressionType {

    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value {
        call_each_variant!(self, p, {
            p.evaluate(tuple, schema)
        })
    }

    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value {
        call_each_variant!(self, p, {
            p.evaluate_join(left_tuple, left_schema, right_tuple, right_schema)
        })
    }

    fn get_children(&self) -> &[ExpressionRef] {
        call_each_variant!(self, p, {
            p.get_children()
        })
    }

    fn get_child_at(&self, child_idx: usize) -> &ExpressionRef {
        call_each_variant!(self, p, {
            p.get_child_at(child_idx)
        })
    }

    fn get_return_type(&self) -> DBTypeId {
        call_each_variant!(self, p, {
            p.get_return_type()
        })
    }
}
