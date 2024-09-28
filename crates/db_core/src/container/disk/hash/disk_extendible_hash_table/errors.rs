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

    #[error("unknown error during init")]
    Unknown,
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InsertionError {
    #[error("Key already exists")]
    KeyAlreadyExists,

    #[error("error during split")]
    InsertionSplitError(InsertionSplitError),

    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("No space left for inserting as the bucket is full and it cannot be splitted again")]
    BucketIsFull,

    #[error("unknown error during insert")]
    Unknown,
}

impl From<SplitError> for InsertionError {
    fn from(value: SplitError) -> Self {
        match value {
            SplitError::BufferPoolError(e) => Self::BufferPoolError(e),
            SplitError::DirectoryIsFull => Self::BucketIsFull,
            SplitError::ReachedRetryLimit(v) => Self::InsertionSplitError(InsertionSplitError::ReachedRetryLimit(v)),
            SplitError::Unknown => Self::InsertionSplitError(InsertionSplitError::Unknown),
        }
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InsertionSplitError {
    #[error("Tried to split bucket for {0} times")]
    ReachedRetryLimit(usize),

    #[error("unknown split during insertion error")]
    Unknown,
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum SplitError {
    #[error("Tried to split bucket for {0} times")]
    ReachedRetryLimit(usize),

    #[error("Directory is full")]
    DirectoryIsFull,

    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown split error")]
    Unknown,
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum LookupError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown error")]
    Unknown,
}
