use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ExtendibleHashBucketPageInsertionErrors {
    #[error("Bucket is full")]
    BucketIsFull,

    #[error("Key already exists")]
    KeyAlreadyExists,

    #[error("unknown extendible hash bucket page error")]
    Unknown,
}
