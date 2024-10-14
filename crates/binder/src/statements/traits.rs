use std::fmt::Debug;
use crate::parse_node_error::ParsePgNodeError;
use crate::statements::StatementType;


pub type StatementTryFromResult<Stmt: Statement> = Result<Stmt, ParsePgNodeError>;

/// This is the BoundStatement in Bustub
pub trait Statement: Debug {
    const TYPE: StatementType;
}
