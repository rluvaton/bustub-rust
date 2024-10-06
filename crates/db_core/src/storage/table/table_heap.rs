use std::sync::Arc;
use parking_lot::Mutex;
use common::config::{PageId, INVALID_PAGE_ID};
use crate::buffer::BufferPoolManager;

/// TODO - implement more from src/include/storage/table/table_heap.h
pub struct TableHeap {
    #[allow(unused)]
    bpm: Arc<BufferPoolManager>,

    /// Default: `INVALID_PAGE_ID`
    #[allow(unused)]
    first_page_id: PageId,

    /// Default: `INVALID_PAGE_ID`
    #[allow(unused)]
    last_page_id: Mutex<PageId>
}

impl TableHeap {

    /// Create a table heap without a transaction. (open table)
    pub fn new(buffer_pool_manager: Arc<BufferPoolManager>) -> Self {
        Self {
            bpm: buffer_pool_manager,
            first_page_id: INVALID_PAGE_ID,
            last_page_id: Mutex::new(INVALID_PAGE_ID)
        }
    }
}
