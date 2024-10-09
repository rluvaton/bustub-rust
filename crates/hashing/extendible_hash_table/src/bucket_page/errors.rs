use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub(crate) enum InsertionErrors {
    #[error("Bucket is full")]
    BucketIsFull,

    #[error("Key already exists")]
    KeyAlreadyExists,

    #[error("unknown extendible hash bucket page error")]
    Unknown,
}