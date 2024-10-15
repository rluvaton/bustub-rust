use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use tuple::Tuple;
use crate::expression_type::ExpressionType;
use crate::traits::{Expression, ExpressionRef, NO_CHILDREN};


/** ComparisonType represents the type of comparison that we want to perform. */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StringExpressionType { Lower, Upper }

impl Display for StringExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StringExpressionType::Lower => write!(f, "lower"),
            StringExpressionType::Upper => write!(f, "upper"),
        }
    }
}

/// StringExpression represents two expressions being computed.
#[derive(Clone, Debug, PartialEq)]
pub struct StringExpression {
    /** The children of this expression. Note that the order of appearance of children may matter. */
    children: Vec<ExpressionRef>,

    expr_type: StringExpressionType,
}

impl StringExpression {
    /** Creates a new comparison expression representing (left comp_type right). */
    pub fn new(arg: ExpressionRef, expr_type: StringExpressionType) -> Self {
        assert_eq!(arg.get_return_type(), DBTypeId::VARCHAR, "Argument must be string");

        Self {
            children: vec![arg],
            expr_type,
        }
    }

}


impl Display for StringExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.expr_type, self.children[0])
    }
}

impl Into<ExpressionType> for StringExpression {
    fn into(self) -> ExpressionType {
        ExpressionType::String(self)
    }
}

impl Expression for StringExpression {
    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value {
        // TODO(student): implement upper / lower.

        let arg = self.children[0].evaluate(tuple, schema);

        // match self.expr_type {
        //     StringExpressionType::Upper => arg.try_cast_as(DBTypeId::VARCHAR).expect("must be able to cast to string").into(),
        // }

        Value::default()
    }

    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value {
        let val = self.children[0].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);

        // TODO(student): implement upper / lower.

        Value::default()
    }

    fn get_children(&self) -> &[ExpressionRef] {
        self.children.as_slice()
    }

    fn get_return_type(&self) -> DBTypeId {
        DBTypeId::VARCHAR
    }
}
