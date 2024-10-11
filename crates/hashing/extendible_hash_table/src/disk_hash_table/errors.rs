use crate::disk_hash_table::logic;

pub use logic::{InsertionError, RemoveError, LookupError, UpdateError};

#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum InitError {
    #[error("buffer pool error")]
    BufferPoolError(#[from] buffer_pool_manager::errors::BufferPoolError),

    #[error("unknown error during init")]
    Unknown,
}



// TODO - add merge errors
