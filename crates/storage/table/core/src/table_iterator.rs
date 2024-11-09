use std::sync::Arc;
use buffer_common::AccessType;
use buffer_pool_manager::BufferPool;
use pages::INVALID_PAGE_ID;
use rid::RID;
use tuple::{Tuple, TupleMeta};
use crate::{TableHeap, TablePage};

pub struct TableIterator<'a> {
    table_heap: &'a TableHeap,
    rid: RID,

    // When creating table iterator, we will record the maximum RID that we should scan.
    // Otherwise we will have dead loops when updating while scanning. (In project 4, update should be implemented as
    // deletion + insertion.)
    stop_at_rid: RID,
}

impl<'a> TableIterator<'a> {
    pub(crate) fn new(table_heap: &'a TableHeap, rid: RID, stop_at_rid: RID) -> Self {
        Self {
            table_heap,
            rid,
            stop_at_rid,
        }
    }
}


impl Iterator for TableIterator<'_> {
    type Item = (TupleMeta, Tuple);

    fn next(&mut self) -> Option<Self::Item> {
        let mut item;

        loop {
            if self.rid.is_invalid() {
                return None;
            }

            // If reached the end, the rid is the same
            if self.rid == self.stop_at_rid {
                return None;
            }

            item = self.table_heap.get_tuple(&self.rid);

            if !item.0.is_deleted {
                break;
            }
            
            // Go to next
            let next_tuple_id = self.rid.get_slot_num() + 1;

            self.rid.set_slot_num(next_tuple_id);
        }

        let page_guard = self.table_heap.bpm.as_ref().expect("Must have BPM").fetch_page_read(self.rid.get_page_id(), AccessType::Unknown).expect("Must be able to fetch page");
        let page = page_guard.cast::<TablePage>();
        let next_tuple_id = self.rid.get_slot_num() + 1;

        if !self.stop_at_rid.is_invalid() {
            assert!(
                // case 1: cursor before the page of the stop tuple
                self.rid.get_page_id() < self.stop_at_rid.get_page_id() ||
                    // case 2: cursor at the page before the tuple
                    (self.rid.get_page_id() == self.stop_at_rid.get_page_id() && next_tuple_id <= self.stop_at_rid.get_slot_num()),
                "iterate out of bound");
        }

        self.rid.set_slot_num(next_tuple_id);

        if self.rid == self.stop_at_rid {
            self.rid.set(INVALID_PAGE_ID, 0);
        } else if next_tuple_id < page.get_num_tuples() {
            // That's fine
        } else {
            let next_page_id = page.get_next_page_id();
            // if next page is invalid, RID is set to invalid page; otherwise, it's the first tuple in that page.
            self.rid.set(next_page_id, 0);
        }

        Some(item)
    }

    // TODO - add next_chunk for fetching a whole page for better performance
}
