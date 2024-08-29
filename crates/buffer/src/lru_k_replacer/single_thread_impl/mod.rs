pub mod implementation;

use std::cell::{Cell, RefCell, UnsafeCell};
use std::cmp::Ordering;
use crate::lru_k_replacer::counter::AtomicU64Counter;
use crate::lru_k_replacer::lru_k_node::LRUKNode;
use common::config::FrameId;
use std::collections::{HashMap};
use std::sync::Arc;
use mut_binary_heap::{BinaryHeap, FnComparator};

type LRUKNodeWrapper = Arc<UnsafeCell<LRUKNode>>;

/**
 * LRUKReplacer implements the LRU-k replacement policy.
 *
 * The LRU-k algorithm evicts a frame whose backward k-distance is maximum
 * of all frames. Backward k-distance is computed as the difference in time between
 * current timestamp and the timestamp of kth previous access.
 *
 * A frame with less than k historical references is given
 * +inf as its backward k-distance. When multiple frames have +inf backward k-distance,
 * classical LRU algorithm is used to choose victim.
 */
#[derive(Clone, Debug)]
pub struct LRUKReplacerImpl {
    // TODO(student): implement me! You can replace these member variables as you like.
    // Remove #[allow(dead_code)] if you start using them.

    /// in cpp it was unordered_map
    pub(crate) node_store: HashMap<FrameId, LRUKNodeWrapper>,

    pub(crate) evictable_heap: BinaryHeap<FrameId, LRUKNodeWrapper, FnComparator<fn(&LRUKNodeWrapper, &LRUKNodeWrapper) -> Ordering>>,

    // TODO - set default to 0
    // #[allow(dead_code)]
    // pub(crate) current_timestamp: isize,

    // TODO - set default to 0
    // #[allow(dead_code)]
    // pub(crate) curr_size: isize,

    pub(crate) replacer_size: usize,

    pub(crate) k: usize,

    // Tracks the number of evictable frames
    pub(crate) evictable_frames: usize,

    pub(crate) history_access_counter: Arc<AtomicU64Counter>,
}

unsafe impl Send for LRUKReplacerImpl {}
