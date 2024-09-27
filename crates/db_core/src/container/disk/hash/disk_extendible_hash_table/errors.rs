use crate::buffer;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum ExtendibleHashTableErrors {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown buffer pool error")]
    Unknown,
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InitError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown buffer pool error")]
    Unknown,
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InsertionError {
    #[error("Key already exists")]
    KeyAlreadyExists,

    #[error("error during split")]
    SplitError(SplitError),

    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),



    #[error("unknown buffer pool error")]
    Unknown,
}

impl From<SplitError> for InsertionError {
    fn from(value: SplitError) -> Self {
        match value {
            SplitError::BufferPoolError(e) => Self::BufferPoolError(e),
            _ => Self::SplitError(value)
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum SplitError {
    #[error("Tried to split bucket for {0} times")]
    ReachedRetryLimit(usize),

    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown buffer pool error")]
    Unknown,
}

