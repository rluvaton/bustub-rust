use pages::PAGE_SIZE;

/// size of buffer pool
pub(crate) const BUFFER_POOL_SIZE: usize = 10;
pub(crate) const LOG_BUFFER_SIZE: usize = (BUFFER_POOL_SIZE + 1) * PAGE_SIZE;  // size of a log buffer in byte

pub(crate) type LogBuffer = [u8; LOG_BUFFER_SIZE];
pub(crate) type FlushBuffer = [u8; LOG_BUFFER_SIZE];
