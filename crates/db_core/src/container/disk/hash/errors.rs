use thiserror::Error;
use crate::buffer;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum HashTableErrors {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::errors::BufferPoolError),

    #[error("unknown buffer pool error")]
    Unknown,
}

pub type HashTableResult<T> = Result<T, HashTableErrors>;
