use common::config::PageId;
use error_utils::{Error};

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
#[error("all available frames are used and not evictable")]
pub(super) struct NoAvailableFrameFound;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
#[error("page id is invalid")]
pub(super) struct InvalidPageId;

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum NewPageError {
    #[error("all frames are used and not evictable")]
    NoAvailableFrameFound,
}

impl From<NoAvailableFrameFound> for NewPageError {
    fn from(_: NoAvailableFrameFound) -> Self {
        NewPageError::NoAvailableFrameFound
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum FetchPageError {
    #[error("Page id is invalid")]
    InvalidPageId,

    #[error("all frames are used and not evictable")]
    NoAvailableFrameFound,
}

impl From<InvalidPageId> for FetchPageError {
    fn from(_: InvalidPageId) -> Self {
        FetchPageError::InvalidPageId
    }
}

impl From<NoAvailableFrameFound> for FetchPageError {
    fn from(_: NoAvailableFrameFound) -> Self {
        FetchPageError::NoAvailableFrameFound
    }
}


#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum DeletePageError {
    #[error("Page id is invalid")]
    InvalidPageId,

    #[error("Page {0} is not evictable")]
    PageIsNotEvictable(PageId),
}

impl From<InvalidPageId> for DeletePageError {
    fn from(_: InvalidPageId) -> Self {
        DeletePageError::InvalidPageId
    }
}

// This is mostly used for consumers that just want to say that there were a problem with
// the buffer pool
#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum UnderlyingBufferPoolError {
    // TODO - add flush to this

    #[error("Error while creating new page in buffer pool")]
    NewPageError(#[from] NewPageError),

    #[error("Error while fetching from buffer pool")]
    FetchPageError(#[from] FetchPageError),

    #[error("Error while deleting page from buffer pool")]
    DeletePageError(#[from] DeletePageError)
}

pub type BufferPoolError = Error<UnderlyingBufferPoolError>;

pub trait MapErrorToBufferPoolError<T> {
    fn map_err_to_buffer_pool_err(self) -> Result<T, BufferPoolError>;
}

impl<T> MapErrorToBufferPoolError<T> for Result<T, NewPageError> {
    fn map_err_to_buffer_pool_err(self) -> Result<T, BufferPoolError> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(UnderlyingBufferPoolError::NewPageError(err).into())
        }
    }
}

impl<T> MapErrorToBufferPoolError<T> for Result<T, FetchPageError> {
    fn map_err_to_buffer_pool_err(self) -> Result<T, BufferPoolError> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(UnderlyingBufferPoolError::FetchPageError(err).into())
        }
    }
}

impl<T> MapErrorToBufferPoolError<T> for Result<T, DeletePageError> {
    fn map_err_to_buffer_pool_err(self) -> Result<T, BufferPoolError> {
        match self {
            Ok(v) => Ok(v),
            Err(err) => Err(UnderlyingBufferPoolError::DeletePageError(err).into())
        }
    }
}
