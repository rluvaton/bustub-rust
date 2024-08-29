use std::cmp::Ordering;
use std::collections::LinkedList;
use chrono::Utc;
use common::config::FrameId;
use crate::lru_k_replacer::counter::{AtomicU64Counter};


pub(crate) type HistoryRecord = (
    // Global counter
    u64,

    // Timestamp
    i64
);

type LRUKNodeCompareItem<'a> = (
    &'a LRUKNode,

    // Interval
    i64
);

const INF_INTERVAL: i64 = i64::MAX;

// In order to avoid getting the current time when wanting to calculate the interval,
// we will use this large number that we will never reach and consider it as current time
const IMAGINARY_NOW: i64 = i64::MAX / 2;

// In order to avoid

#[derive(Clone, Debug)]
pub struct LRUKNode {

    // Remove #[allow(dead_code)] if you start using them. Feel free to change the member variables as you want.

    /// History of last seen K timestamps of this page. Least recent timestamp stored in front
    /// in cpp it was std::list<size_t>
    /// # Performance improvement idea
    /// we can use a fixed size array and have index to the start of the data and index to the end of the data
    /// adding new items will set at the end index and increment that index
    /// when reached the end of the array going to the start and moving the first item to be start index + 1
    #[allow(dead_code)]
    pub(crate) history: LinkedList<HistoryRecord>,

    #[allow(dead_code)]
    pub(crate) k: usize,

    #[allow(dead_code)]
    pub(crate) frame_id: FrameId,

    #[allow(dead_code)]
    pub(crate) is_evictable: bool,

    pub(crate) interval: i64,
}

impl LRUKNode {
    pub(crate) fn new(k: usize, frame_id: FrameId, counter: &AtomicU64Counter) -> Self {
        assert!(k > 0, "K > 0");

        let mut history = LinkedList::new();

        let now = Self::get_new_access_record_now(counter);
        let interval = IMAGINARY_NOW - now.1;

        history.push_back(now);

        LRUKNode {
            history,
            k,
            frame_id,
            is_evictable: false,
            interval,
        }
    }

    pub(crate) fn marked_accessed(&mut self, counter: &AtomicU64Counter) {


        // Why we only don't need to recalculate the pair duration
        // now: x
        // item1: i_1
        // item2: i_2
        // itemN: i_n

        // Example for having 5 items
        // = (x - i_5) + (i_5 - i_4) + (i_4 - i_3) + (i_3 - i_2) + (i_2 - i_1)
        // = x - i_5 + i_5 - i_4 + i_4 - i_3 + i_3 - i_2 + i_2 - i_1
        // = x + 0 + 0 + 0 - i_1
        // = x - i_1

        // When need to remove the first item and add new item instead at the beginning
        // = (x - i_6) + (i_6 - i_5) + (i_5 - i_4) + (i_4 - i_3) + (i_3 - i_2)
        // = x - i_6 + i_6 - i_5 + i_5 - i_4 + i_4 - i_3 + i_3 - i_2
        // = x + 0 + 0 + 0 - i_2
        // = x - i_2

        // when need to just add item without removing
        // = (x - i_7) + (i_7 - i_6) + (i_6 - i_5) + (i_5 - i_4) + (i_4 - i_3) + (i_3 - i_2)
        // = x - i_7 + i_7 - i_6 + i_6 - i_5 + i_5 - i_4 + i_4 - i_3 + i_3 - i_2
        // = x + 0 + 0 + 0 - i_2
        // = x - i_2


        let new_val = Self::get_new_access_record_now(counter);

        // If reached the size, remove the first item and add to the end
        if self.history.len() >= self.k {
            let removed = self.history.pop_front().unwrap();

            self.interval += removed.1;
            self.interval -= new_val.1;
        }

        self.history.push_back(new_val);
    }

    pub(crate) fn calculate_intervals(&self, now: i64) -> i64 {
        if self.history.len() < self.k {
            // If less than the number of records just make it the largest so it would be first to evict
            return INF_INTERVAL;
        }

        let to_now_duration = now - self.history.back().expect("should have at least 1").1;

        self.interval + to_now_duration
    }

    #[inline]
    pub(crate) fn get_interval(&self) -> i64 {
        if self.history.len() < self.k {
            return (
                // If less than the number of records just make it the largest so it would be first to evict
                INF_INTERVAL -

                    // Fallback to LRU
                    // Not using timestamp as counter will not depend on the machine clock precision

                    // Subtracting the current message id to make sure most recent (largest message id) will have smaller value than the least recent node (smallest message id)
                    self.get_current_message_id() as i64
            );
        }

        self.interval
    }

    pub(crate) fn get_current_timestamp(&self) -> i64 {
        self.history.back().expect("History can never be empty").1
    }

    pub(crate) fn get_current_message_id(&self) -> u64 {
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

    pub(crate) fn get_current_time() -> i64 {
        Utc::now().timestamp_micros()
    }

    /// Return which node is first_to_evict
    ///
    /// # Algorithm
    /// if both a and b does not have enough history records, go by LRU, least recently used would be first
    /// if only one of them does not have enough history record make it first
    /// if both have enough, make the one that has the largest gap between access times the first
    ///
    /// # Arguments
    ///
    /// * `a`: item 1
    /// * `b`: item 2
    ///
    /// returns: Ordering
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub(crate) fn next_to_evict_compare(a: &LRUKNodeCompareItem, b: &LRUKNodeCompareItem) -> Ordering {
        let k = a.0.k;

        let does_a_not_having_enough_history = a.0.history.len() < k;
        let does_b_not_having_enough_history = b.0.history.len() < k;

        // If both not having enough history access
        if does_a_not_having_enough_history && does_b_not_having_enough_history {
            // Fallback to LRU
            // Not using timestamp as counter will not depend on the machine clock precision
            return a.0.get_current_message_id().cmp(&b.0.get_current_message_id());
        }

        // If 'A' or 'B' not having enough history, making them first
        if does_a_not_having_enough_history || does_b_not_having_enough_history {
            return does_b_not_having_enough_history.cmp(&does_a_not_having_enough_history);
        }

        // We want larger value to be at the beginning - reversed
        // larger value at the beginning as this mean that the difference between access time is the largest
        b.1.cmp(&a.1)
    }

    /// Return which node is first_to_evict
    ///
    /// # Algorithm
    /// if both a and b does not have enough history records, go by LRU, least recently used would be first
    /// if only one of them does not have enough history record make it first
    /// if both have enough, make the one that has the largest gap between access times the first
    ///
    /// # Arguments
    ///
    /// * `a`: item 1
    /// * `b`: item 2
    ///
    /// returns: Ordering
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        self.get_interval().cmp(&other.get_interval())
    }
}
