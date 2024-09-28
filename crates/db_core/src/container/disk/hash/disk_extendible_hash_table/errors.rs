use crate::buffer;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InitError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer::BufferPoolError),

    #[error("unknown error during init")]
    Unknown,
}


// TODO - add merge errors
