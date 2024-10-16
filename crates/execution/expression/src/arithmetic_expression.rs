use crate::expression_type::ExpressionType;
use crate::traits::{Expression, ExpressionRef};
use catalog_schema::Schema;
use data_types::{DBTypeId, Value};
use std::fmt::{Debug, Display, Formatter};
use tuple::Tuple;


/** ArithmeticType represents the type of computation that we want to perform. */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArithmeticExpressionType {
    /// Plus, e.g. `a + b`
    Plus,
    /// Minus, e.g. `a - b`
    Minus,
    /// Multiply, e.g. `a * b`
    Multiply,
    /// Divide, e.g. `a / b`
    Divide,
    /// Modulo, e.g. `a % b`
    Modulo,
}

impl ArithmeticExpressionType {
    fn calc(&self, lhs: Value, rhs: Value) -> Value {
        match self {
            ArithmeticExpressionType::Plus => lhs + rhs,
            ArithmeticExpressionType::Minus => lhs - rhs,
            ArithmeticExpressionType::Multiply => lhs * rhs,
            ArithmeticExpressionType::Divide => lhs / rhs,
            ArithmeticExpressionType::Modulo => lhs % rhs
        }
    }
}

impl Display for ArithmeticExpressionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ArithmeticExpressionType::Plus => write!(f, "+"),
            ArithmeticExpressionType::Minus => write!(f, "-"),
            ArithmeticExpressionType::Multiply => write!(f, "*"),
            ArithmeticExpressionType::Divide => write!(f, "/"),
            ArithmeticExpressionType::Modulo => write!(f, "%"),
        }
    }
}

/// ArithmeticExpression represents two expressions being computed, ONLY SUPPORT INTEGER FOR NOW.
#[derive(Clone, Debug, PartialEq)]
pub struct ArithmeticExpression {
    /// From Abstract expression

    /** The children of this expression. Note that the order of appearance of children may matter. */
    children: Vec<ExpressionRef>,

    /** The return type of this expression. */
    ret_type: DBTypeId,

    //

    compute_type: ArithmeticExpressionType,
}

impl ArithmeticExpression {
    /** Creates a new comparison expression representing (left comp_type right). */
    pub fn new(left: ExpressionRef, right: ExpressionRef, compute_type: ArithmeticExpressionType) -> Self {
        // TODO - actually not only int is supported
        assert_eq!(left.get_return_type(), DBTypeId::INT, "Only support int for now");
        assert_eq!(right.get_return_type(), DBTypeId::INT, "Only support int for now");

        Self {
            children: vec![left, right],
            ret_type: DBTypeId::INT,
            compute_type,
        }
    }

}


impl Display for ArithmeticExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.children[0], self.compute_type, self.children[1])
    }
}

impl Into<ExpressionType> for ArithmeticExpression {
    fn into(self) -> ExpressionType {
        ExpressionType::Arithmetic(self)
    }
}

impl Expression for ArithmeticExpression {
    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value {
        let lhs = self.children[0].evaluate(tuple, schema.clone());
        let rhs = self.children[1].evaluate(tuple, schema);

        self.compute_type.calc(lhs, rhs)
    }

    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value {
        let lhs = self.children[0].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);
        let rhs = self.children[1].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);

        self.compute_type.calc(lhs, rhs)
    }

    fn get_children(&self) -> &[ExpressionRef] {
        self.children.as_slice()
    }

    fn get_return_type(&self) -> DBTypeId {
        self.ret_type
    }
}
