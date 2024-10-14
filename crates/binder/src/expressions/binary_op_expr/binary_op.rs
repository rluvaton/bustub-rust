use sqlparser::ast::BinaryOperator;
use crate::try_from_ast_error::ParseASTError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BinaryOp {
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
    /// Greater than, e.g. `a > b`
    Gt,
    /// Less than, e.g. `a < b`
    Lt,
    /// Greater equal, e.g. `a >= b`
    GtEq,
    /// Less equal, e.g. `a <= b`
    LtEq,
    /// Equal, e.g. `a = b`
    Eq,
    /// Not equal, e.g. `a <> b`
    NotEq,
    /// And, e.g. `a AND b`
    And,
    /// Or, e.g. `a OR b`
    Or,
}

impl TryFrom<&sqlparser::ast::BinaryOperator> for BinaryOp {
    type Error = ParseASTError;

    fn try_from(value: &BinaryOperator) -> Result<Self, Self::Error> {
        Ok(match value {
            BinaryOperator::Plus => BinaryOp::Plus,
            BinaryOperator::Minus => BinaryOp::Minus,
            BinaryOperator::Multiply => BinaryOp::Multiply,
            BinaryOperator::Divide => BinaryOp::Divide,
            BinaryOperator::Modulo => BinaryOp::Modulo,
            BinaryOperator::Gt => BinaryOp::Gt,
            BinaryOperator::Lt => BinaryOp::Lt,
            BinaryOperator::GtEq => BinaryOp::GtEq,
            BinaryOperator::LtEq => BinaryOp::LtEq,
            BinaryOperator::Eq => BinaryOp::Eq,
            BinaryOperator::NotEq => BinaryOp::NotEq,
            BinaryOperator::And => BinaryOp::And,
            BinaryOperator::Or => BinaryOp::Or,
            _ => return Err(ParseASTError::Unimplemented(format!("{} not supported", value)))
        })
    }
}
