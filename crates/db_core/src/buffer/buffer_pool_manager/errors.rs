use error_utils::Error;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum UnderlyingBufferPoolError {
    #[error("Page id is invalid")]
    InvalidPageId,
    #[error("all frames are used and not evictable")]
    NoAvailableFrameFound,
    #[error("unknown buffer pool error")]
    Unknown,
}

pub type BufferPoolError = Error<UnderlyingBufferPoolError>;

pub type BufferPoolResult<T> = Result<T, BufferPoolError>;
