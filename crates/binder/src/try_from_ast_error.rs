
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum ParseASTError {
    #[error("node is incompatible")]
    IncompatibleType,

    #[error("Failed to parse {0}")]
    FailedParsing(String),

    #[error("Using unimplemented features. {0}")]
    Unimplemented(String),

    #[error("Invalid SQL: {0}")]
    InvalidSQL(#[from] sqlparser::parser::ParserError),

    #[error("{0}")]
    Other(String),
}

pub type ParseASTResult<T> = Result<T, ParseASTError>;

pub trait ParseASTResultExt {
    fn is_incompatible_type(&self) -> bool;
}

impl<T> ParseASTResultExt for ParseASTResult<T> {
    fn is_incompatible_type(&self) -> bool {
        self.as_ref().is_err_and(|err| !matches!(err, ParseASTError::IncompatibleType))
    }
}


#[macro_export]
macro_rules! fallback_on_incompatible_2_args {
    ($func:ident, $arg1:expr, $arg2:expr, {$($struct:ty),*}) => {
        {
            $(
                let mapped = <$struct>::$func($arg1, $arg2);
                if mapped.as_ref().is_err_and(|err| !matches!(err, ParseASTError::IncompatibleType)) {
                    return Err(mapped.unwrap_err());
                }
                if mapped.is_ok() {
                    return mapped.map(|item| item.into());
                }
            )*
        }
    }
}


