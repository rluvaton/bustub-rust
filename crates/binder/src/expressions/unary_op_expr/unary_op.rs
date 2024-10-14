use crate::try_from_ast_error::ParseASTError;
use sqlparser::ast::UnaryOperator;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnaryOp {
    /// Not, e.g. `NOT(true)`
    Not,
}

impl TryFrom<&sqlparser::ast::UnaryOperator> for UnaryOp {
    type Error = ParseASTError;

    fn try_from(value: &UnaryOperator) -> Result<Self, Self::Error> {
        Ok(match value {
            UnaryOperator::Not => UnaryOp::Not,
            _ => return Err(ParseASTError::Unimplemented(format!("{} not supported", value)))
        })
    }
}
