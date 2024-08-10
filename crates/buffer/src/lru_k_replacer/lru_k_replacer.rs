use std::collections::HashMap;
use std::sync::Mutex;
use common::config::FrameId;
use crate::lru_k_replacer::counter::AtomicU64Counter;
use crate::lru_k_replacer::lru_k_node::LRUKNode;

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
pub struct LRUKReplacer {
    // TODO(student): implement me! You can replace these member variables as you like.
    // Remove #[allow(dead_code)] if you start using them.

    /// in cpp it was unordered_map
    /// # Performance improvement idea
    /// the following idea will improve eviction time but decrease record access time
    ///
    /// split to 2 parts, evictable and non-evictable
    /// evictable is sorted by first to evict and each time updating access it will update the location of the frame id
    #[allow(dead_code)]
    pub(crate) node_store: HashMap<FrameId, LRUKNode>,

    // TODO - set default to 0
    // #[allow(dead_code)]
    // pub(crate) current_timestamp: isize,

    // TODO - set default to 0
    // #[allow(dead_code)]
    // pub(crate) curr_size: isize,

    #[allow(dead_code)]
    pub(crate) replacer_size: usize,

    #[allow(dead_code)]
    pub(crate) k: isize,

    // TODO - if using remove the option?
    #[allow(dead_code)]
    pub(crate) latch: Option<Mutex<()>>,

    // Tracks the number of evictable frames
    pub(crate) evictable_frames: isize,

    pub(crate) history_access_counter: AtomicU64Counter,
}


