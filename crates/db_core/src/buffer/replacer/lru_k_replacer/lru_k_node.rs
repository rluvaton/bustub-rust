use std::cmp::Ordering;
use data_structures::FixedSizeLinkedList;

use common::config::FrameId;

use super::counter::AtomicI64Counter;

// Global counter
// Not using timestamp as it is slower
type HistoryRecord = i64;

const INF_COUNTER: i64 = i64::MAX;

// In order to avoid getting the last message id ever created (right) when wanting to calculate the interval,
// we will use this large number that we will never reach and consider it as current value
const IMAGINARY_NOW: i64 = i64::MAX / 2;

#[derive(Clone, Debug)]
pub(in crate::buffer) struct LRUKNode {

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
    pub(super) fn new(k: usize, frame_id: FrameId, counter: &AtomicI64Counter) -> Self {
        assert!(k > 0, "K > 0");

        let mut history = FixedSizeLinkedList::with_capacity(k);

        let now = Self::get_new_access_record(counter);
        let interval = if k != 1 { LRUKNode::calculate_interval_for_less_than_k(now) } else { LRUKNode::calculate_interval_full_access_history(&now) };

        history.push_back(now);

        LRUKNode {
            history,
            frame_id,
            is_evictable: false,
            interval,
        }
    }

    pub(super) fn marked_accessed(&mut self, counter: &AtomicI64Counter) {

        // LRU-K evicts the page whose K-th most recent access is furthest in the past.
        // So we only need to calculate

        let new_val = Self::get_new_access_record(counter);
        let removed = self.history.push_back_rotate(new_val);

        // If reached the size, remove the first item and add to the end
        if let Some(removed) = removed {
            self.interval += removed - new_val;
        } else {
            // If now full set the interval value to be the correct K-distance so the next calls to mark as accessed are faster
            if self.history.is_full() {
                self.interval = LRUKNode::calculate_interval_full_access_history(self.history.front().unwrap());
            } else {
                self.interval = LRUKNode::calculate_interval_for_less_than_k(new_val)
            }
        }
    }

    #[inline(always)]
    fn get_interval(&self) -> i64 {
        self.interval
    }

    #[inline(always)]
    fn calculate_interval_for_less_than_k(last_access: HistoryRecord) -> i64 {
        // Fallback to LRU
        // Subtracting the current access record to make sure most recent (largest access record) will have smaller value than the least recent node (smallest access record)
        INF_COUNTER - last_access
    }

    #[inline(always)]
    fn calculate_interval_full_access_history(first_access: &HistoryRecord) -> i64 {
        // Calculate the distance from now to the first access (the k-access)
        IMAGINARY_NOW - first_access
    }

    fn get_new_access_record(counter: &AtomicI64Counter) -> HistoryRecord {
        counter.get_next()
    }

    #[inline(always)]
    pub(super) fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
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
        let counter = AtomicI64Counter::default();
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
