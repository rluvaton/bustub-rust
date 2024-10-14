use std::fmt::Debug;
use crate::try_from_ast_error::TryFromASTError;
use crate::statements::StatementType;


pub type StatementTryFromResult<Stmt: Statement> = Result<Stmt, TryFromASTError>;

/// This is the BoundStatement in Bustub
pub trait Statement: Debug + for<'a> TryFrom<&'a sqlparser::ast::Statement, Error=TryFromASTError> {
    const TYPE: StatementType;
}
