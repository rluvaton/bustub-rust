use sqllogictest::TestError;
use sqlparser::parser::ParserError;
use thiserror::Error;

pub type Result<T, E = BustubSqlLogicTestError> = std::result::Result<T, E>;

type BustubError = error_utils::anyhow::Error;

/// DataFusion sql-logicaltest error
#[derive(Debug, Error)]
pub enum BustubSqlLogicTestError {
    /// Error from sqllogictest-rs
    #[error("SqlLogicTest error(from sqllogictest-rs crate): {0}")]
    SqlLogicTest(#[from] TestError),
    /// Error from bustub
    #[error("Bustub error: {0}")]
    Bustub(#[from] BustubError),
    /// Error returned when SQL is syntactically incorrect.
    #[error("SQL Parser error: {0}")]
    Sql(#[from] ParserError),
    /// Generic error
    #[error("Other Error: {0}")]
    Other(String),
}

impl From<String> for BustubSqlLogicTestError {
    fn from(value: String) -> Self {
        BustubSqlLogicTestError::Other(value)
    }
}
