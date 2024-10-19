use crate::expression_type::ExpressionType;
use crate::traits::{Expression, ExpressionRef};
use catalog_schema::Schema;
use data_types::{BooleanType, DBTypeId, DBTypeIdImpl, Value};
use std::fmt::{Debug, Display, Formatter};
use tuple::Tuple;


/** ArithmeticType represents the type of logic operation that we want to perform. */
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicType { And, Or }

impl LogicType {
    fn cmp(&self, lhs: Value, rhs: Value) -> Value {
        let (lhs, rhs) = match (lhs.get_value(), rhs.get_value()) {
            (DBTypeIdImpl::BOOLEAN(lhs_bool), DBTypeIdImpl::BOOLEAN(rhs_bool)) => (lhs_bool.get_as_bool(), rhs_bool.get_as_bool()),
            (_, _) => unreachable!("We already check the values are boolean")
        };

        let value = match self {
            LogicType::And => {
                match (lhs, rhs) {
                    (Some(false), _) | (_, Some(false)) => Some(false),
                    (Some(true), Some(true)) => Some(true),
                    (None, _) | (_, None) => None,
                }
            }
            LogicType::Or => {
                match (lhs, rhs) {
                    (Some(false), Some(false)) => Some(false),
                    (Some(true), _) | (_, Some(true)) => Some(true),
                    (None, _) | (_, None) => None,
                }
            }
        };

        BooleanType::from(value).into()
    }
}

impl Display for LogicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicType::And => write!(f, "and"),
            LogicType::Or => write!(f, "or"),
        }
    }
}

/// LogicExpression represents two expressions being computed.
#[derive(Clone, Debug, PartialEq)]
pub struct LogicExpression {
    /// From Abstract expression

    /** The children of this expression. Note that the order of appearance of children may matter. */
    children: Vec<ExpressionRef>,

    //

    logic_type: LogicType,
}

impl LogicExpression {
    /** Creates a new comparison expression representing (left comp_type right). */
    pub fn new(left: ExpressionRef, right: ExpressionRef, logic_type: LogicType) -> Self {
        assert_eq!(left.get_return_type(), DBTypeId::BOOLEAN, "Cant do and/or on non booleans");
        assert_eq!(right.get_return_type(), DBTypeId::BOOLEAN, "Cant do and/or on non booleans");

        Self {
            children: vec![left, right],
            logic_type,
        }
    }

}


impl Display for LogicExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.children[0], self.logic_type, self.children[1])
    }
}

impl Into<ExpressionType> for LogicExpression {
    fn into(self) -> ExpressionType {
        ExpressionType::Logic(self)
    }
}

impl Expression for LogicExpression {
    fn evaluate(&self, tuple: &Tuple, schema: &Schema) -> Value {
        let lhs = self.children[0].evaluate(tuple, schema);
        let rhs = self.children[1].evaluate(tuple, schema);

        self.logic_type.cmp(lhs, rhs)
    }

    fn evaluate_join(&self, left_tuple: &Tuple, left_schema: &Schema, right_tuple: &Tuple, right_schema: &Schema) -> Value {
        let lhs = self.children[0].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);
        let rhs = self.children[1].evaluate_join(left_tuple, left_schema, right_tuple, right_schema);

        self.logic_type.cmp(lhs, rhs)
    }

    fn get_children(&self) -> &[ExpressionRef] {
        self.children.as_slice()
    }

    fn get_return_type(&self) -> DBTypeId {
        DBTypeId::BOOLEAN
    }
}
