use crate::lru_k_replacer::LRUKReplacer;
use common::config::{AtomicPageId, FrameId, PageId};
use parking_lot::Mutex;
use recovery::LogManager;
use std::cell::UnsafeCell;
use std::collections::{HashMap, LinkedList};
use std::sync::Arc;
use storage::{DiskScheduler, Page};
use crate::buffer_pool_manager::manager_stats::BufferPoolManagerStats;

/**
 * BufferPoolManager reads disk pages to and from its internal buffer pool.
 */
pub struct BufferPoolManager {
    /// Number of pages in the buffer pool
    /// This will not change after initial set
    /// TODO - remove pub(crate) and expose getter to avoid user setting the value
    pub(crate) pool_size: usize,

    // TODO - panic will release the parking lot Mutex lock which can leave undesired state
    //        replace to original Mutex
    /// This latch protects the root level data until we get to the actual page instance, this is here to be the gateway in the inner data
    pub(crate) root_level_latch: Mutex<()>,

    /// This is just container to the inner buffer pool manager, so we can do locking with better granularity
    /// as it allow for multiple mutable reference at the same time but it's ok as we are managing it
    pub(crate) inner: UnsafeCell<InnerBufferPoolManager>,

    pub(crate) stats: BufferPoolManagerStats,
}

unsafe impl Sync for BufferPoolManager { }

/**
 * BufferPoolManager reads disk pages to and from its internal buffer pool.
 */
pub(crate) struct InnerBufferPoolManager {

    /** The next page id to be allocated  */
    pub(crate) next_page_id: AtomicPageId,

    /** Array of buffer pool pages. */
    // The index is the frame_id
    pub(crate) pages: Vec<Page>,

    /// Pointer to the disk scheduler.
    /// This is mutex to avoid writing and reading the same page twice
    pub(crate) disk_scheduler: Arc<Mutex<DiskScheduler>>,

    /** Pointer to the log manager. Please ignore this for P1. */
    // LogManager *log_manager_ __attribute__((__unused__));
    pub(crate) log_manager: Option<LogManager>,

    /// Page table for keeping track of buffer pool pages.
    ///
    /// ## Original type:
    /// ```cpp
    /// std::unordered_map<page_id_t, frame_id_t> page_table_;
    /// ```
    pub(crate) page_table: HashMap<PageId, FrameId>,

    /** Replacer to find unpinned pages for replacement. */
    // std::unique_ptr<LRUKReplacer> replacer_;
    pub(crate) replacer: LRUKReplacer,

    /** List of free frames that don't have any pages on them. */
    // std::list<frame_id_t> free_list_;
    pub(crate) free_list: LinkedList<FrameId>,
}
