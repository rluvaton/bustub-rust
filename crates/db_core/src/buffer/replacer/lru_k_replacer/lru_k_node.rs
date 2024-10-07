use std::cmp::Ordering;
use data_structures::FixedSizeLinkedList;
use chrono::Utc;

use common::config::FrameId;

use super::counter::AtomicU64Counter;

type HistoryRecord = (
    // Global counter
    u64,

    // Timestamp
    i64
);


const INF_INTERVAL: i64 = i64::MAX;

// In order to avoid getting the current time when wanting to calculate the interval,
// we will use this large number that we will never reach and consider it as current time
const IMAGINARY_NOW: i64 = i64::MAX / 2;

// In order to avoid

#[derive(Clone, Debug)]
pub(in crate::buffer) struct LRUKNode {

    // Remove #[allow(dead_code)] if you start using them. Feel free to change the member variables as you want.

    /// History of last seen K timestamps of this page. Least recent timestamp stored in front
    /// in cpp it was std::list<size_t>
    /// # Performance improvement idea
    /// we can use a fixed size array and have index to the start of the data and index to the end of the data
    /// adding new items will set at the end index and increment that index
    /// when reached the end of the array going to the start and moving the first item to be start index + 1
    history: FixedSizeLinkedList<HistoryRecord>,

    frame_id: FrameId,

    is_evictable: bool,

    interval: i64,
}

impl LRUKNode {
    pub(super) fn new(k: usize, frame_id: FrameId, counter: &AtomicU64Counter) -> Self {
        assert!(k > 0, "K > 0");

        let mut history = FixedSizeLinkedList::with_capacity(k);

        let now = Self::get_new_access_record_now(counter);
        let interval = IMAGINARY_NOW - now.1;

        history.push_back(now);

        LRUKNode {
            history,
            frame_id,
            is_evictable: false,
            interval,
        }
    }

    pub(super) fn marked_accessed(&mut self, counter: &AtomicU64Counter) {

        // LRU-K evicts the page whose K-th most recent access is furthest in the past.
        // So we only need to calculate

        let new_val = Self::get_new_access_record_now(counter);
        let removed = self.history.push_back_rotate(new_val);

        // If reached the size, remove the first item and add to the end
        if let Some(removed) = removed {
            self.interval += removed.1 - new_val.1;
        }
    }

    #[inline(always)]
    fn get_interval(&self) -> i64 {
        if !self.history.is_full() {
            return INF_INTERVAL -

                // Fallback to LRU
                // Not using timestamp as counter will not depend on the machine clock precision

                // Subtracting the current message id to make sure most recent (largest message id) will have smaller value than the least recent node (smallest message id)
                self.get_current_message_id() as i64;
        }

        self.interval
    }

    fn get_current_message_id(&self) -> u64 {
        self.history.back().expect("History can never be empty").0
    }

    fn get_new_access_record_now(counter: &AtomicU64Counter) -> HistoryRecord {
        (
            // Counter,
            counter.get_next(),

            // Timestamp
            Self::get_current_time(),
        )
    }

    fn get_current_time() -> i64 {
        Utc::now().timestamp_micros()
    }

    pub(super) fn cmp(&self, other: &Self) -> Ordering {
        self.get_interval().cmp(&other.get_interval())
    }

    #[inline]
    pub(super) fn get_frame_id(&self) -> FrameId {
        self.frame_id
    }

    #[inline]
    pub(super) fn is_evictable(&self) -> bool {
        self.is_evictable
    }

    #[inline]
    pub(super) fn set_evictable(&mut self, evictable: bool) {
        self.is_evictable = evictable;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_frame_ids(nodes: &Vec<LRUKNode>) -> Vec<FrameId> {
        nodes.iter().map(|node| node.frame_id).collect()
    }

    /// Get frame ids of vector of nodes so that the first node is the next to be evicted and last node is the last node to be evicted
    fn get_frame_ids_sorted_by_evictable(nodes: &Vec<LRUKNode>) -> Vec<FrameId> {
        let mut nodes = nodes.clone();

        // Sort in reverse order
        nodes.sort_by(|a, b| b.cmp(a));

        nodes.iter().map(|node| node.frame_id).collect()
    }

    #[test]
    fn should_order_by_record_access() {
        let counter = AtomicU64Counter::default();
        let mut original_nodes = {
            let mut nodes = vec![];
            for i in 0..5 {
                nodes.push(LRUKNode::new(3, i, &counter))
            }

            nodes
        };

        assert_eq!(get_frame_ids(&original_nodes), vec![0, 1, 2, 3, 4]);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![0, 1, 2, 3, 4]);

        // node 2 should now be the least evicted
        original_nodes[2].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![0, 1, 3, 4, 2]);

        // node 4 should now be the least evicted
        original_nodes[4].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![0, 1, 3, 2, 4]);

        original_nodes[0].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 3, 2, 4, 0]);

        // Reaching K access history
        original_nodes[2].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 3, 4, 0, 2]);

        original_nodes[3].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        // Reaching K access history, 3 is after 2 as it was created after 2
        original_nodes[3].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 2, 3]);

        original_nodes[2].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        original_nodes[3].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 2, 3]);

        original_nodes[2].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        original_nodes[2].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        original_nodes[2].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        original_nodes[3].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        original_nodes[3].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 3, 2]);

        // First access is after first access of 2
        original_nodes[3].marked_accessed(&counter);
        assert_eq!(get_frame_ids_sorted_by_evictable(&original_nodes), vec![1, 4, 0, 2, 3]);
    }
}
