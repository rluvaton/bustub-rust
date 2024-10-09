use super::AtomicI64Counter;
use data_structures::{DoubleEndedList, FixedSizeLinkedList};

// Global counter
// Not using timestamp as it is slower
type HistoryRecord = i64;

const INF_COUNTER: i64 = i64::MAX;

// In order to avoid getting the last message id ever created (right) when wanting to calculate the interval,
// we will use this large number that we will never reach and consider it as current value
const IMAGINARY_NOW: i64 = i64::MAX / 2;

pub const NO_HEAP_POS: usize = usize::MAX;

// TODO - make it possible to reuse easily
#[derive(Clone, Debug)]
pub(crate) struct LRUKNode {
    /// History of last seen K timestamps of this page. Least recent timestamp stored in front
    /// in cpp it was std::list<size_t>
    history: FixedSizeLinkedList<HistoryRecord>,

    pub(super) interval: i64,
    heap_pos: usize,
}

impl LRUKNode {

    pub(super) fn create_invalid(k: usize) -> Self {
        assert!(k > 0, "K > 0");

        LRUKNode {
            history: FixedSizeLinkedList::with_capacity(k),
            interval: 0,
            heap_pos: NO_HEAP_POS,
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

    #[must_use]
    #[inline(always)]
    pub(super) fn has_heap_pos(&self) -> bool {
        self.heap_pos != NO_HEAP_POS
    }

    #[inline(always)]
    pub(super) fn remove_heap_pos(&mut self) {
        self.heap_pos = NO_HEAP_POS;
    }

    #[inline(always)]
    pub(super) unsafe fn get_heap_pos_unchecked(&self) -> usize {
        self.heap_pos
    }

    #[inline(always)]
    pub(super) fn set_heap_pos(&mut self, heap_pos: usize) {
        self.heap_pos = heap_pos;
    }

    pub(super) fn reuse(&mut self, counter: &AtomicI64Counter) {
        self.history.start_over();

        let now = Self::get_new_access_record(counter);
        self.interval = if self.history.capacity() != 1 { LRUKNode::calculate_interval_for_less_than_k(now) } else { LRUKNode::calculate_interval_full_access_history(&now) };

        self.history.push_back(now);
        self.heap_pos = NO_HEAP_POS;
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
                self.interval = Self::calculate_interval_full_access_history(self.history.front().unwrap());
            } else {
                self.interval = Self::calculate_interval_for_less_than_k(new_val)
            }
        }
    }

    #[inline]
    pub(super) fn is_evictable(&self) -> bool {
        self.heap_pos != NO_HEAP_POS
    }
}
