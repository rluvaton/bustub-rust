use pages::PAGE_SIZE;

/// size of buffer pool
pub const BUFFER_POOL_SIZE: usize = 10;
pub const LOG_BUFFER_SIZE: usize = (BUFFER_POOL_SIZE + 1) * PAGE_SIZE;  // size of a log buffer in byte

pub type LogBuffer = [u8; LOG_BUFFER_SIZE];
pub type FlushBuffer = [u8; LOG_BUFFER_SIZE];
