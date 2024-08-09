use std::collections::HashMap;
use std::sync::Mutex;
use common::config::FrameId;
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

    // in cpp it was unordered_map
    #[allow(dead_code)]
    node_store: HashMap<FrameId, LRUKNode>,

    // TODO - set default to 0
    #[allow(dead_code)]
    current_timestamp: isize,

    // TODO - set default to 0
    #[allow(dead_code)]
    curr_size: isize,

    #[allow(dead_code)]
    replacer_size: isize,

    #[allow(dead_code)]
    k: isize,

    // TODO - if using, replace the mutex value with something that is needed
    #[allow(dead_code)]
    latch: Mutex<u8>,
}


