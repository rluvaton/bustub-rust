// Heap implementation greatly influenced by https://github.com/Wasabi375/mut-binary-heap
use super::{LRUKNode, AtomicI64Counter};
use common::config::FrameId;
use core::mem::swap;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::vec;
use super::LRUNode;
use bit_vec::BitVec;


/// This max heap for LRU-K + node store
///
//
/// Heap for evictable LRU-K nodes for best performance for finding evictable frames
/// This is mutable Heap to allow for updating LRU-K Node without removing and reinserting
/// TODO - implement proper debug
#[derive(Clone, Debug)]
pub(super) struct LRUKReplacerStore {
    /// Bit vector for whether a frame exists or not
    ///
    /// TODO - maybe remove this and add it to the lru-k node for locality?
    can_use: BitVec,

    /// Heap of frame ids where in the top of the heap, the frame that are about to be evicted
    ///
    /// We are not using HashMap for performance as we can avoid the hashing by simple index lookup
    next_frame_to_evict_heap: Vec<FrameId>,

    all: Vec<LRUKNode>,
    _not_sync: PhantomData<std::cell::Cell<()>>,
}


unsafe impl Send for LRUKReplacerStore {}

impl LRUKReplacerStore {
    #[must_use]
    pub fn with_capacity(k: usize, capacity: usize) -> Self {
        LRUKReplacerStore {
            can_use: BitVec::from_elem(capacity, false),
            next_frame_to_evict_heap: Vec::with_capacity(capacity),
            all: vec![LRUKNode::create_invalid(k); capacity],
            _not_sync: PhantomData::default(),
        }
    }

    pub fn get_order_of_eviction(mut self) -> Vec<FrameId> {
        let mut frames = vec![];

        while let Some(frame_id) = self.remove_next_evictable_frame() {
            frames.push(frame_id)
        }

        frames
    }

    pub fn add_non_evictable_node(&mut self, frame_id: FrameId, history_access_counter: &Arc<AtomicI64Counter>) {
        let node = &mut self.all[frame_id as usize];
        node.reuse(history_access_counter);
        self.can_use.set(frame_id as usize, true);

        debug_assert_eq!(node.is_evictable(), false);
    }

    pub fn remove_node_if_evictable(&mut self, frame_id: FrameId) -> bool {
        // No need to check if item exists because when we remove frame we also mark it as non evictable
        if !self.all[frame_id as usize].is_evictable() {
            return false;
        }

        self.can_use.set(frame_id as usize, false);

        self.mark_frame_as_not_evictable(&frame_id);

        true
    }

    #[inline(always)]
    pub fn has_node(&self, frame_id: FrameId) -> bool {
        self.can_use[frame_id as usize]
    }

    /// # Safety
    /// node must exists
    #[inline(always)]
    pub unsafe fn is_existing_node_evictable_unchecked(&self, frame_id: FrameId) -> bool {
        self.all[frame_id as usize].is_evictable()
    }

    #[inline(always)]
    pub fn get_node(&mut self, frame_id: FrameId) -> Option<&mut LRUKNode> {
        if !self.can_use[frame_id as usize] {
            return None;
        }

        let node = &mut self.all[frame_id as usize];

        Some(node)
    }

    /// mark existing frame as newly evictable, the frame must not already be evictable
    ///
    /// This will push to the evictable binary heap
    ///
    /// # Time complexity
    ///
    /// The expected cost of `push`, averaged over every possible ordering of
    /// the elements being pushed, and over a sufficiently large number of
    /// pushes, is *O*(1). This is the most meaningful cost metric when pushing
    /// elements that are *not* already in any sorted pattern.
    ///
    /// The time complexity degrades if elements are pushed in predominantly
    /// ascending order. In the worst case, elements are pushed in ascending
    /// sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    /// containing *n* elements.
    #[inline(always)]
    pub fn mark_frame_as_evictable(&mut self, frame_id: FrameId) {
        debug_assert_eq!(self.all[frame_id as usize].is_evictable(), false, "Frame must be evictable");

        let old_len = self.len();
        self.next_frame_to_evict_heap.push(frame_id);
        self.all[frame_id as usize].set_heap_pos(old_len);
        // SAFETY: Since we pushed a new item it means that
        //  old_len = self.len() - 1 < self.len()
        unsafe { self.sift_up(0, old_len) };
    }

    /// Remove the next evictable frame and return it or `None` if no frame is evictable
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    #[inline(always)]
    pub fn remove_next_evictable_frame(&mut self) -> Option<FrameId> {
        self.next_frame_to_evict_heap.pop().map(|mut item| {
            // NOTE: we can't just use self.is_empty here, because that will
            //  trigger a debug_assert that keys and data are equal lenght.
            if !self.next_frame_to_evict_heap.is_empty() {
                swap(&mut item, &mut self.next_frame_to_evict_heap[0]);
                // SAFETY: !self.is_empty() means that self.len() > 0
                unsafe { self.sift_down_to_bottom(0) };
            }
            self.can_use.set(item as usize, false);

            item
        })
    }

    /// Removes evictable frame from the evictable heap
    ///
    /// The frame must be evictable for performance reasons (to avoid the extra check)
    #[inline(always)]
    pub fn mark_frame_as_not_evictable(&mut self, key: &FrameId) {
        debug_assert_eq!(self.all[*key as usize].is_evictable(), true);

        let pos = unsafe { self.all[*key as usize].get_heap_pos_unchecked() };
        let mut item = self.next_frame_to_evict_heap.pop().unwrap();
        if !self.next_frame_to_evict_heap.is_empty() && pos < self.next_frame_to_evict_heap.len() {
            swap(&mut item, &mut self.next_frame_to_evict_heap[pos]);
            // SAFETY: !self.is_empty && pos < self.data.len()
            unsafe { self.sift_down_to_bottom(pos) };
        }
        self.all[item as usize].remove_heap_pos();
    }

    /// Updates the binary heap after frame_id interval updates
    ///
    /// # Time complexity
    ///
    /// This function runs in *O*(*log* n) time.
    pub fn update_after_evictable(&mut self, key: FrameId) {
        let pos = unsafe { self.all[key as usize].get_heap_pos_unchecked() };
        let pos_after_sift_up = unsafe { self.sift_up(0, pos) };
        if pos_after_sift_up != pos {
            return;
        }
        unsafe {
            self.sift_down(pos);
        }
    }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.data.len()
        let frame_id = self.next_frame_to_evict_heap[pos];

        let mut pos = pos;
        while pos > start {
            let parent = (pos - 1) / 2;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().

            if self.compares_le(frame_id, self.next_frame_to_evict_heap[parent]) {
                break;
            }

            // SAFETY: Same as above
            pos = self.move_hole_to_new_position(pos, parent);
        }

        self.fill_hole(pos, frame_id);

        pos
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.data.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
        let frame_id = self.next_frame_to_evict_heap[pos];

        let mut pos = pos;
        let mut child = 2 * pos + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // compare with the greater of the two children
            // SAFETY: child < end - 1 < self.data.len() and
            //  child + 1 < end <= self.data.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += self.compares_le(self.next_frame_to_evict_heap[child], self.next_frame_to_evict_heap[child + 1]) as usize;

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or the old child+1
            //  We already proven that both are < self.data.len() and != hole.pos()
            if self.compares_ge(frame_id, self.next_frame_to_evict_heap[child]) {
                return;
            }

            pos = self.move_hole_to_new_position(pos, child);
            child = 2 * pos + 1;
        }

        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child == end - 1 < self.data.len().
        if child == end - 1 && self.compares_lt(frame_id, self.next_frame_to_evict_heap[child]) {
            // SAFETY: child is already proven to be a valid index and
            //  child == 2 * hole.pos() + 1 != hole.pos().
            pos = self.move_hole_to_new_position(pos, child);
        }

        self.fill_hole(pos, frame_id);
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
        let len = self.next_frame_to_evict_heap.len();
        // SAFETY: pos < len is guaranteed by the caller and
        //  obviously len = self.data.len() <= self.len().
        unsafe { self.sift_down_range(pos, len) };
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    unsafe fn sift_down_to_bottom(&mut self, mut pos: usize) {
        let end = self.next_frame_to_evict_heap.len();
        let start = pos;

        let frame_id = self.next_frame_to_evict_heap[pos];

        let mut pos = pos;
        let mut child = 2 * pos + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // SAFETY: child < end - 1 < self.data.len() and
            //  child + 1 < end <= self.data.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { self.compares_le(self.next_frame_to_evict_heap[child], self.next_frame_to_evict_heap[child + 1]) } as usize;

            pos = self.move_hole_to_new_position(pos, child);
            child = 2 * pos + 1;
        }

        if child == end - 1 {
            // SAFETY: child == end - 1 < self.data.len(), so it's a valid index
            //  and child == 2 * hole.pos() + 1 != hole.pos().
            pos = self.move_hole_to_new_position(pos, child);
        }
        self.fill_hole(pos, frame_id);


        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        unsafe { self.sift_up(start, pos) };
    }

    // Check if less than or equal to
    fn compares_le(&self, a: FrameId, b: FrameId) -> bool {
        unsafe { self.get_node_by_frame_unchecked(a).interval <= self.get_node_by_frame_unchecked(b).interval }
    }

    // Check if less than
    fn compares_lt(&self, a: FrameId, b: FrameId) -> bool {
        unsafe { self.get_node_by_frame_unchecked(a).interval < self.get_node_by_frame_unchecked(b).interval }
    }

    // Check if greater than or equal to
    fn compares_ge(&self, a: FrameId, b: FrameId) -> bool {
        unsafe { self.get_node_by_frame_unchecked(a).interval >= self.get_node_by_frame_unchecked(b).interval }
    }

    unsafe fn get_node_by_frame_unchecked(&self, frame_id: FrameId) -> &LRUKNode {
        assert!(self.can_use[frame_id as usize]);
        &self.all[frame_id as usize]
    }

    /// Returns the length of the binary heap.
    ///
    #[must_use]
    fn len(&self) -> usize {
        self.next_frame_to_evict_heap.len()
    }

    /// Checks if the binary heap is empty.
    ///
    #[must_use]
    fn is_empty(&self) -> bool {
        self.next_frame_to_evict_heap.is_empty()
    }


    /// Move hole to new location and return the updated position
    ///
    /// # Safety
    ///
    /// target_position must be within the data slice and not equal to pos.
    #[inline]
    fn move_hole_to_new_position(&mut self, hole_pos: usize, target_position: usize) -> usize {
        debug_assert_ne!(target_position, hole_pos);
        debug_assert!(target_position < self.next_frame_to_evict_heap.len());
        // update target index in key map
        let target_frame = self.next_frame_to_evict_heap[target_position];

        debug_assert!(self.all[target_frame as usize].has_heap_pos(), "Hole can only exist for key values pairs, that are already part of the heap.");
        // Update the position of the node to point to the new location
        self.all[target_frame as usize].set_heap_pos(hole_pos);

        // move target into hole
        self.next_frame_to_evict_heap.swap(target_position, hole_pos);

        // update hole position
        target_position
    }

    /// restore the slice by filling the hole
    /// position with the value that was originally removed.
    fn fill_hole(&mut self, pos: usize, frame_id: FrameId) {
        // fill the hole again
        self.next_frame_to_evict_heap[pos] = frame_id;

        debug_assert!(self.all[frame_id as usize].has_heap_pos(), "Hole can only exist for key values pairs, that are already part of the heap.");

        // Update the position
        self.all[frame_id as usize].set_heap_pos(pos);
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    fn is_normal<T: Send + Unpin>() {}

    #[test]
    fn check_is_send_unpin() {
        is_normal::<LRUKReplacerStore>();
        assert!(true);
    }

    fn assert_key_map_valid(bh: &LRUKReplacerStore) {
        let mut expected_keys = HashMap::new();
        for (i, kv) in bh.next_frame_to_evict_heap.iter().enumerate() {
            expected_keys.insert(kv.clone(), i);
        }

        for key_index in &expected_keys {
            let key = key_index.0;
            let index = *key_index.1;
            unsafe { assert_eq!(bh.all[*key as usize].get_heap_pos_unchecked(), index); }
        }
        let keys_len = bh.all.iter().filter(|item| item.has_heap_pos()).count();
        assert_eq!(keys_len, expected_keys.len());
    }

    // #[test]
    // fn valid_key_map() {
    //     let counter = AtomicI64Counter::default();
    //     let mut heap: LRUKReplacerStore = LRUKReplacerStore::with_capacity(10);
    //
    //     assert_key_map_valid(&heap);
    //
    //     heap.push(0, LRUKNode::new(1, &counter));
    //
    //     assert_key_map_valid(&heap);
    //
    //     heap.push(1, LRUKNode::new(10, &counter));
    //     heap.push(2, 15);
    //     heap.push(3, 5);
    //     heap.push(4, 8);
    //
    //     assert_key_map_valid(&heap);
    //
    //     assert_eq!(heap.pop_with_key(), Some((2, 15)));
    //
    //     assert_key_map_valid(&heap);
    //
    //     assert_eq!(heap.pop_with_key(), Some((1, 10)));
    //     assert_eq!(heap.pop_with_key(), Some((4, 8)));
    //
    //     heap.push(5, 2);
    //
    //     assert_key_map_valid(&heap);
    //
    //     assert_eq!(heap.pop_with_key(), Some((3, 5)));
    //     assert_eq!(heap.pop_with_key(), Some((5, 2)));
    //     assert_eq!(heap.pop_with_key(), Some((0, 0)));
    //
    //     assert_key_map_valid(&heap);
    //
    //     assert_eq!(heap.pop_with_key(), None);
    //
    //     assert_key_map_valid(&heap);
    // }
    //
    // #[test]
    // fn valid_key_map_after_clear() {
    //     let mut heap: LRUKReplacerStore = LRUKReplacerStore::with_capacity(10);
    //
    //     assert_key_map_valid(&heap);
    //
    //     heap.push(0, 0);
    //
    //     assert_key_map_valid(&heap);
    //
    //     heap.push(1, 10);
    //     heap.push(2, 15);
    //     heap.push(3, 5);
    //     heap.push(4, 8);
    //
    //     assert_key_map_valid(&heap);
    //
    //     heap.clear();
    //
    //     assert_key_map_valid(&heap);
    //     assert_eq!(heap.len(), 0);
    // }
}
