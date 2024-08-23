use crate::lru_k_replacer::LRUKReplacer;
use common::config::{AtomicPageId, FrameId, PageId};
use parking_lot::Mutex;
use recovery::LogManager;
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;
use storage::{DiskScheduler, Page};

/**
 * BufferPoolManager reads disk pages to and from its internal buffer pool.
 */
#[derive(Clone)]
pub struct BufferPoolManager {
    /** Number of pages in the buffer pool. */
    pub(crate) pool_size: usize,
    /** This latch protects shared data structures. We recommend updating this comment to describe what it protects. */
    pub(crate) latch: Arc<Mutex<InnerBufferPoolManager>>,
}

/**
 * BufferPoolManager reads disk pages to and from its internal buffer pool.
 */
pub(crate) struct InnerBufferPoolManager {

    /** The next page id to be allocated  */
    pub(crate) next_page_id: AtomicPageId,
    /** Array of buffer pool pages. */
    // The index is the frame_id
    pub(crate) pages: Vec<Page>,
    /** Pointer to the disk scheduler. */
    pub(crate) disk_scheduler: Arc<Mutex<DiskScheduler>>,

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
    pub(crate) free_list: LinkedList<FrameId>,
}
