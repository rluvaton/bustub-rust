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
    pub(crate) k: isize,

    #[allow(dead_code)]
    pub(crate) frame_id: FrameId,

    // TODO - set default to false
    #[allow(dead_code)]
    pub(crate) is_evictable: bool,
}

impl LRUKNode {
    pub(crate) fn new(k: isize, frame_id: FrameId, counter: &AtomicU64Counter) -> Self {
        assert!(k > 0, "K >= 0");

        let mut history = LinkedList::new();

        history.push_back(Self::get_new_access_record_now(counter));

        LRUKNode {
            history,
            k,
            frame_id,
            is_evictable: false,
        }
    }

    pub(crate) fn marked_accessed(&mut self, counter: &AtomicU64Counter) {
        let new_val = Self::get_new_access_record_now(counter);

        // If reached the size, remove the first item and add to the end
        if self.history.len() >= self.k as usize {
            self.history.pop_front();
        }

        self.history.push_back(new_val);
    }

    pub(crate) fn calculate_intervals(&self, now: i64) -> i64 {
        if self.history.len() < self.k as usize {
            // If less than the number of records just make it the largest so it would be first to evict
            return i64::MAX;
        }

        let mut diff = 0;
        let mut last = self.history.front().expect("K cant be 0");
        let mut last = last.1;

        for (_id, timestamp) in self.history.iter().skip(1) {
            diff += timestamp - last;
            last = *timestamp;
        }

        diff + (now - last)
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
        let k = a.0.k as usize;

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
}
