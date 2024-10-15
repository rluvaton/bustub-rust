use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use tuple::Tuple;
use crate::expression_type::ExpressionType;
use crate::traits::{Expression, ExpressionRef, NO_CHILDREN};


/** ComparisonType represents the type of comparison that we want to perform. */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisonType { Equal, NotEqual, LessThan, LessThanOrEqual, GreaterThan, GreaterThanOrEqual }

impl Display for ComparisonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // ArithmeticExpressionType::Minus => write!(f, "-"),
            ComparisonType::Equal => write!(f, "="),
            ComparisonType::NotEqual => write!(f, "!="),
            ComparisonType::LessThan => write!(f, "<"),
            ComparisonType::LessThanOrEqual => write!(f, "<="),
            ComparisonType::GreaterThan => write!(f, ">"),
            ComparisonType::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

/// ComparisonExpression represents two expressions being compared.
#[derive(Clone, Debug, PartialEq)]
pub struct ComparisonExpression {
    /// From Abstract expression

    /** The children of this expression. Note that the order of appearance of children may matter. */
    children: Vec<ExpressionRef>,

    /** The return type of this expression. */
    ret_type: DBTypeId,

    //

    comp_type: ComparisonType,
}

impl ComparisonExpression {
    /** Creates a new comparison expression representing (left comp_type right). */
    pub fn new(left: ExpressionRef, right: ExpressionRef, comp_type: ComparisonType) -> Self {
        Self {
            children: vec![left, right],
            ret_type: DBTypeId::BOOLEAN,
            comp_type,
        }
    }

}


impl Display for ComparisonExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.children[0], self.comp_type, self.children[1])
    }
}

impl Into<ExpressionType> for ComparisonExpression {
    fn into(self) -> ExpressionType {
        ExpressionType::Comparison(self)
    }
}

impl Expression for ComparisonExpression {
    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value {
        let lhs = self.children[0].evaluate(tuple, schema);
        let rhs = self.children[1].evaluate(tuple, schema);

        match self.comp_type {
            ComparisonType::Equal => (lhs == rhs).into(),
            ComparisonType::NotEqual => (lhs != rhs).into(),
            ComparisonType::LessThan => (lhs < rhs).into(),
            ComparisonType::LessThanOrEqual => (lhs <= rhs).into(),
            ComparisonType::GreaterThan => (lhs > rhs).into(),
            ComparisonType::GreaterThanOrEqual => (lhs >= rhs).into(),
        }
    }

    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value {
        let lhs = self.children[0].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);
        let rhs = self.children[1].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);

        match self.comp_type {
            ComparisonType::Equal => (lhs == rhs).into(),
            ComparisonType::NotEqual => (lhs != rhs).into(),
            ComparisonType::LessThan => (lhs < rhs).into(),
            ComparisonType::LessThanOrEqual => (lhs <= rhs).into(),
            ComparisonType::GreaterThan => (lhs > rhs).into(),
            ComparisonType::GreaterThanOrEqual => (lhs >= rhs).into(),
        }
    }

    fn get_children(&self) -> &[ExpressionRef] {
        self.children.as_slice()
    }

    fn get_return_type(&self) -> DBTypeId {
        self.ret_type
    }
}
