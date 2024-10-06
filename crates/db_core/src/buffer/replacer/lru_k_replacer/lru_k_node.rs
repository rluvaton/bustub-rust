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
