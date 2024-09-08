use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU32};

/** Cycle detection is performed every CYCLE_DETECTION_INTERVAL milliseconds. */
// extern std::chrono::milliseconds cycle_detection_interval;

/** True if logging should be enabled, false otherwise. */
// extern std::atomic<bool> enable_logging;

/** If ENABLE_LOGGING is true, the log should be flushed to disk every LOG_TIMEOUT. */
// extern std::chrono::duration<int64_t> log_timeout;

pub const INVALID_PAGE_ID: PageId = -1;                                           // invalid page id
pub const INVALID_TXN_ID: TxnId = -1;                                            // invalid transaction id
pub const INVALID_LSN: LSN = -1;                                               // invalid log sequence number
pub const INVALID_TIMESTAMP: Timestamp = -1;
pub const HEADER_PAGE_ID: PageId = 0;                                             // the header page id
pub const BUSTUB_PAGE_SIZE: usize = 4096;                                        // size of a data page in byte
pub const BUFFER_POOL_SIZE: usize = 10;                                          // size of buffer pool
pub const LOG_BUFFER_SIZE: usize = (BUFFER_POOL_SIZE + 1) * BUSTUB_PAGE_SIZE;  // size of a log buffer in byte
pub const BUCKET_SIZE: usize = 50;                                               // size of extendible hash bucket
pub const LRUK_REPLACER_K: usize = 10;  // lookback window for lru-k replacer

pub type FrameId = i32; // frame id type (in cpp it was `frame_id_t`)
pub type PageId = i32; // page id type (in cpp it was `page_id_t`)
pub type AtomicPageId = AtomicI32; // std::atomic<page_id_t>
pub type TxnId = i64; // transaction id type (in cpp it was `txn_id_t`)
pub type AtomicTxnId = AtomicI64; // std::atomic<txn_id_t>
pub type LSN = i32; // log sequence number type (in cpp it was `lsn_t`)
pub type AtomicLSN = AtomicI32; // std::atomic<lsn_t>
pub type SlotOffset = isize; // slot offset type (in cpp it was `slot_offset_t = size_t`)
pub type OID = u16; // (in cpp it was `oid_t`)
pub type Timestamp = i64;
pub type AtomicTimestamp = AtomicI64;
pub type TableOID = u32; // (in cpp it was `table_oid_t`)
pub type AtomicTableOID = AtomicU32;
pub type IndexOID = u32; // (in cpp it was `index_oid_t`)
pub type AtomicIndexOID = AtomicU32;


pub type PageData = [u8; BUSTUB_PAGE_SIZE];

pub const TXN_START_ID: TxnId = 1 << 62; // first txn id

pub const VARCHAR_DEFAULT_LENGTH: i32 = 128;  // default length for varchar when constructing the column
