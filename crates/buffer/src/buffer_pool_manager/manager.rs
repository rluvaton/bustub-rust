
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use common::config::{AtomicPageId, FrameId, PageId};
use recovery::LogManager;
use storage::{DiskScheduler, Page};
use crate::lru_k_replacer::LRUKReplacer;

/**
 * BufferPoolManager reads disk pages to and from its internal buffer pool.
 */
pub struct BufferPoolManager {
    /** Number of pages in the buffer pool. */
    pub(crate) pool_size: usize,
    /** The next page id to be allocated  */
    // TODO - default 0
    pub(crate) next_page_id: AtomicPageId,

    /** Array of buffer pool pages. */
    pub(crate) pages: Vec<Page>,
    /** Pointer to the disk sheduler. */
    pub(crate) disk_scheduler: Arc<DiskScheduler>,

    /** Pointer to the log manager. Please ignore this for P1. */
    // LogManager *log_manager_ __attribute__((__unused__));
    pub(crate) log_manager: Option<LogManager>,

    /** Page table for keeping track of buffer pool pages. */
    // std::unordered_map<page_id_t, frame_id_t> page_table_;
    pub(crate) page_table: HashMap<PageId, FrameId>,

    /** Replacer to find unpinned pages for replacement. */
    // std::unique_ptr<LRUKReplacer> replacer_;
    pub(crate) replacer: LRUKReplacer,

    /** List of free frames that don't have any pages on them. */
    // std::list<frame_id_t> free_list_;
    pub(crate) free_list: Vec<FrameId>,
    /** This latch protects shared data structures. We recommend updating this comment to describe what it protects. */
    pub(crate) latch: Mutex<()>,
}
