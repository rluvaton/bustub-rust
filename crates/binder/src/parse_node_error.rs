
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum ParsePgNodeError {
    #[error("node is incompatible")]
    IncompatibleType,

    #[error("Failed to parse {0}")]
    FailedParsing(String),

    #[error("Using unimplemented features {0}")]
    Unimplemented(String),

    #[error("{0}")]
    Other(String),
}
