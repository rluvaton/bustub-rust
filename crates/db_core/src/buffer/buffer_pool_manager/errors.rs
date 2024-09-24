use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum BufferPoolError {
    #[error("Page id is invalid")]
    InvalidPageId,
    #[error("all frames are used and not evictable")]
    NoAvailableFrameFound,
    #[error("unknown buffer pool error")]
    Unknown,
}

pub type BufferPoolResult<T> = Result<T, BufferPoolError>;
