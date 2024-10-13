use std::fmt::Debug;
use crate::statements::StatementType;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum ParsePgNodeError {
    #[error("node is incompatible")]
    IncompatibleType,

    #[error("Failed to parse {0}")]
    FailedParsing(String),
}

pub type StatementTryFromResult<Stmt: Statement> = Result<Stmt, ParsePgNodeError>;

pub trait Statement: Debug + for<'a> TryFrom<pg_query::NodeRef<'a>, Error = ParsePgNodeError> {
    const TYPE: StatementType;

    // type PgNode: pg_parse::ast::Node;

    // TODO - add from
}
