use std::cell::UnsafeCell;
use std::sync::Arc;
use data_structures::FixedSizeLinkedList;

use super::counter::AtomicI64Counter;

// Global counter
// Not using timestamp as it is slower
type HistoryRecord = i64;

const INF_COUNTER: i64 = i64::MAX;

// In order to avoid getting the last message id ever created (right) when wanting to calculate the interval,
// we will use this large number that we will never reach and consider it as current value
const IMAGINARY_NOW: i64 = i64::MAX / 2;

pub(super) type LRUKNodeWrapper = Arc<UnsafeCell<LRUKNode>>;

// TODO - make it possible to reuse easily
#[derive(Clone, Debug)]
pub(in crate::buffer) struct LRUKNode {
    can_use: bool,

    /// History of last seen K timestamps of this page. Least recent timestamp stored in front
    /// in cpp it was std::list<size_t>
    history: FixedSizeLinkedList<HistoryRecord>,

    is_evictable: bool,

    pub(super) interval: i64,
}

impl LRUKNode {
    pub(super) fn new(k: usize, counter: &AtomicI64Counter) -> LRUKNodeWrapper {
        assert!(k > 0, "K > 0");

        let mut history = FixedSizeLinkedList::with_capacity(k);

        let now = Self::get_new_access_record(counter);
        let interval = if k != 1 { LRUKNode::calculate_interval_for_less_than_k(now) } else { LRUKNode::calculate_interval_full_access_history(&now) };

        history.push_back(now);
        Arc::new(UnsafeCell::new(
            LRUKNode {
                can_use: true,
                history,
                is_evictable: false,
                interval,
            }))
    }

    pub(super) fn reuse(&mut self, k: usize, counter: &AtomicI64Counter) {
        self.history.start_over();

        let now = Self::get_new_access_record(counter);
        self.interval = if k != 1 { LRUKNode::calculate_interval_for_less_than_k(now) } else { LRUKNode::calculate_interval_full_access_history(&now) };

        self.history.push_back(now);
        self.can_use = true;
        self.is_evictable = false;
    }

    pub(super) fn reuse_wrapper(node_wrapper: &LRUKNodeWrapper, k: usize, counter: &AtomicI64Counter) {
        unsafe { (*node_wrapper.get()).reuse(k, counter) }
    }

    pub(super) fn inactive(node_wrapper: &LRUKNodeWrapper) {
        unsafe { (*node_wrapper.get()).can_use = false }
    }

    pub(super) fn is_usable(node_wrapper: &LRUKNodeWrapper) -> bool {
        unsafe { (*node_wrapper.get()).can_use }
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

    #[inline]
    pub(super) fn is_evictable(&self) -> bool {
        self.is_evictable
    }

    #[inline]
    pub(super) fn set_evictable(&mut self, evictable: bool) {
        self.is_evictable = evictable;
    }
}
