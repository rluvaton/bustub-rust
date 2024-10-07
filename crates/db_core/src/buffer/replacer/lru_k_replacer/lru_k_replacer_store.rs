use common::config::FrameId;
use core::mem::{swap, ManuallyDrop};
use core::ptr;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::vec;
use super::lru_k_node::LRUKNodeWrapper;

// ###################################################################################
// Copied from https://github.com/Wasabi375/mut-binary-heap and modified to our needs
// ###################################################################################

/// This max heap for LRU-K + node store
///
//
/// Heap for evictable LRU-K nodes for best performance for finding evictable frames
/// This is mutable Heap to allow for updating LRU-K Node without removing and reinserting
/// TODO - implement proper debug
#[derive(Clone, Debug)]
pub(super) struct LRUKReplacerStore {
    /// The key for the node store is the frame_id which is also the index
    /// We are not using HashMap for performance as we can avoid the hashing by simple index lookup
    data: Vec<FrameId>,

    all: Vec<(Option<LRUKNodeWrapper>, Option<usize>)>,
    _not_sync: PhantomData<std::cell::Cell<()>>,
}

unsafe impl Send for LRUKReplacerStore {}

impl LRUKReplacerStore {

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        LRUKReplacerStore {
            data: Vec::with_capacity(capacity),
            all: vec![(None, None); capacity],
            _not_sync: PhantomData::default(),
        }
    }

    // This is for when the item is missing
    pub fn add_node(&mut self, frame_id: FrameId, item: LRUKNodeWrapper, evictable: bool) {
        self.all[frame_id as usize].0.replace(item);

        if evictable {
            self.push_evictable(frame_id);
        }
    }

    // This is for when the item is missing
    pub fn remove_node(&mut self, frame_id: FrameId) -> Option<LRUKNodeWrapper> {
        let removed = self.all[frame_id as usize].0.take()?;

        self.remove_evictable(&frame_id);

        Some(removed)
    }

    pub fn get_node(&self, frame_id: FrameId) -> Option<LRUKNodeWrapper> {
        self.all[frame_id as usize].0.clone()
    }

    /// Pop from top of the heap and remove the node as well
    pub fn pop_node(&mut self) -> Option<(FrameId, LRUKNodeWrapper)>  {
        let (frame_id, _) = self.pop_evictable_with_key()?;

        // Remove frame
        self.all[frame_id as usize].0.take().map(|node| (frame_id, node))
    }

    /**
    Pushes an item onto the binary heap.

    If the heap did not have this key present, [None] is returned.

    If the heap did have this key present, the value is updated, and the old
    value is returned. The key is not updated, though; this matters for
    types that can be `==` without being identical. For more information see
    the documentation of [HashMap::insert].

    # Time complexity

    The expected cost of `push`, averaged over every possible ordering of
    the elements being pushed, and over a sufficiently large number of
    pushes, is *O*(1). This is the most meaningful cost metric when pushing
    elements that are *not* already in any sorted pattern.

    The time complexity degrades if elements are pushed in predominantly
    ascending order. In the worst case, elements are pushed in ascending
    sorted order and the amortized cost per push is *O*(log(*n*)) against a heap
    containing *n* elements.

    The worst case cost of a *single* call to `push` is *O*(*n*). The worst case
    occurs when capacity is exhausted and needs a resize. The resize cost
    has been amortized in the previous figures.
    */
    pub fn push_evictable(&mut self, frame_id: FrameId) {
        if let Some(pos) = self.all[frame_id as usize].1 {
            let mut old = std::mem::replace(&mut self.data[pos], frame_id);
            // NOTE: the swap is required in order to keep the guarantee
            // that the key is not replaced by a second push.
            // I would prefer replacing the key, but that is not supported by
            // [HashMap]
            std::mem::swap(&mut old, &mut self.data[pos]);
            self.update_after_evictable(&old);
        } else {
            let old_len = self.len();
            self.data.push(frame_id);
            self.all[frame_id as usize].1.replace(old_len);
            // SAFETY: Since we pushed a new item it means that
            //  old_len = self.len() - 1 < self.len()
            unsafe { self.sift_up(0, old_len) };
        }
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop_evictable(&mut self) -> Option<LRUKNodeWrapper> {
        self.pop_evictable_with_key().map(|kv| kv.1)
    }

    /// Removes the greatest item from the binary heap and returns it as a key-value pair,
    /// or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a heap containing *n* elements is *O*(log(*n*)).
    pub fn pop_evictable_with_key(&mut self) -> Option<(FrameId, LRUKNodeWrapper)> {
        let item = self.data.pop().map(|mut item| {
            // NOTE: we can't just use self.is_empty here, because that will
            //  trigger a debug_assert that keys and data are equal lenght.
            if !self.data.is_empty() {
                swap(&mut item, &mut self.data[0]);
                // SAFETY: !self.is_empty() means that self.len() > 0
                unsafe { self.sift_down_to_bottom(0) };
            }
            item
        });

        item.as_ref().and_then(|&kv| self.all[kv as usize].1.take());
        item.map(|frame_id| (frame_id, self.all[frame_id as usize].0.clone().unwrap()))
    }

    /// Returns `true` if the heap contains a value for the given key.
    ///
    /// # Time complexity
    ///
    /// This method runs in *O*(1) time.
    ///
    pub fn contains_key(&self, key: &FrameId) -> bool {
        self.all[*key as usize].1.is_some()
    }

    /// Removes a key from the heap, returning the `(key, value)` if the key
    /// was previously in the heap.
    ///
    pub fn remove_evictable(&mut self, key: &FrameId) {
        if let Some(pos) = self.all[*key as usize].1 {
            let item = self.data.pop().map(|mut item| {
                if !self.data.is_empty() && pos < self.data.len() {
                    swap(&mut item, &mut self.data[pos]);
                    // SAFETY: !self.is_empty && pos < self.data.len()
                    unsafe { self.sift_down_to_bottom(pos) };
                }
                item
            });

            // Remove the usize as well
            item.as_ref().and_then(|&kv| self.all[kv as usize].1.take());
        }
    }

    /// Updates the binary heap after the value behind this key was modified.
    ///
    /// This is called by [push] if the key already existed and also by [RefMut].
    ///
    /// This function will panic if the key is not part of the binary heap.
    /// A none panicing alternative is to check with [BinaryHeapIndexKeys::contains_key]
    /// or using [BinaryHeapIndexKeys::get_mut] instead.
    ///
    /// # Time complexity
    ///
    /// This function runs in *O*(*log* n) time.
    fn update_after_evictable(&mut self, key: &FrameId) {
        let pos = self.all[*key as usize].1.unwrap();
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
    unsafe fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        // Take out the value at `pos` and create a hole.
        // SAFETY: The caller guarantees that pos < self.data.len()
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.all, pos) };

        while hole.pos() > start {
            let parent = (hole.pos() - 1) / 2;

            // SAFETY: hole.pos() > start >= 0, which means hole.pos() > 0
            //  and so hole.pos() - 1 can't underflow.
            //  This guarantees that parent < hole.pos() so
            //  it's a valid index and also != hole.pos().
            if LRUKReplacerStore::compares_le(hole.element(), unsafe { hole.get(parent) })
            {
                break;
            }

            // SAFETY: Same as above
            unsafe { hole.move_to(parent) };
        }

        hole.pos()
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `pos < end <= self.data.len()`.
    unsafe fn sift_down_range(&mut self, pos: usize, end: usize) {
        // SAFETY: The caller guarantees that pos < end <= self.data.len().
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.all, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // compare with the greater of the two children
            // SAFETY: child < end - 1 < self.data.len() and
            //  child + 1 < end <= self.data.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { LRUKReplacerStore::compares_le(hole.get(child), hole.get(child + 1)) } as usize;

            // if we are already in order, stop.
            // SAFETY: child is now either the old child or the old child+1
            //  We already proven that both are < self.data.len() and != hole.pos()
            if LRUKReplacerStore::compares_ge(hole.element(), unsafe { hole.get(child) })
            {
                return;
            }

            // SAFETY: same as above.
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        // SAFETY: && short circuit, which means that in the
        //  second condition it's already true that child == end - 1 < self.data.len().
        if child == end - 1
            && LRUKReplacerStore::compares_lt(hole.element(), unsafe { hole.get(child) })
        {
            // SAFETY: child is already proven to be a valid index and
            //  child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
    }

    /// # Safety
    ///
    /// The caller must guarantee that `pos < self.data.len()`.
    unsafe fn sift_down(&mut self, pos: usize) {
        let len = self.data.len();
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
        let end = self.data.len();
        let start = pos;

        // SAFETY: The caller guarantees that pos < self.data.len().
        let mut hole = unsafe { Hole::new(&mut self.data, &mut self.all, pos) };
        let mut child = 2 * hole.pos() + 1;

        // Loop invariant: child == 2 * hole.pos() + 1.
        while child <= end.saturating_sub(2) {
            // SAFETY: child < end - 1 < self.data.len() and
            //  child + 1 < end <= self.data.len(), so they're valid indexes.
            //  child == 2 * hole.pos() + 1 != hole.pos() and
            //  child + 1 == 2 * hole.pos() + 2 != hole.pos().
            // FIXME: 2 * hole.pos() + 1 or 2 * hole.pos() + 2 could overflow
            //  if T is a ZST
            child += unsafe { LRUKReplacerStore::compares_le(hole.get(child), hole.get(child + 1)) } as usize;

            // SAFETY: Same as above
            unsafe { hole.move_to(child) };
            child = 2 * hole.pos() + 1;
        }

        if child == end - 1 {
            // SAFETY: child == end - 1 < self.data.len(), so it's a valid index
            //  and child == 2 * hole.pos() + 1 != hole.pos().
            unsafe { hole.move_to(child) };
        }
        pos = hole.pos();
        drop(hole);

        // SAFETY: pos is the position in the hole and was already proven
        //  to be a valid index.
        unsafe { self.sift_up(start, pos) };
    }

    // Check if less than or equal to
    fn compares_le(a: &LRUKNodeWrapper, b: &LRUKNodeWrapper) -> bool {
        unsafe { (*a.get()).interval <= (&*b.get()).interval }
    }

    // Check if less than
    fn compares_lt(a: &LRUKNodeWrapper, b: &LRUKNodeWrapper) -> bool {
        unsafe { (*a.get()).interval < (&*b.get()).interval }
    }

    // Check if greater than or equal to
    fn compares_ge(a: &LRUKNodeWrapper, b: &LRUKNodeWrapper) -> bool {
        unsafe { (*a.get()).interval >= (&*b.get()).interval }
    }

    /// Returns the length of the binary heap.
    ///
    #[must_use]
    pub fn len(&self) -> usize {
        debug_assert!(self.data.len() == self.data.len());
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    ///
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}


/// Hole represents a hole in a slice i.e., an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a> {
    data: &'a mut [FrameId],
    all: &'a mut [(Option<LRUKNodeWrapper>, Option<usize>)],
    elt: ManuallyDrop<FrameId>,
    pos: usize,
}

impl<'a> Hole<'a> {
    /// Create a new `Hole` at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [FrameId], all: &'a mut [(Option<LRUKNodeWrapper>, Option<usize>)], pos: usize) -> Self {
        debug_assert!(pos < data.len());
        // SAFE: pos should be inside the slice
        let frame_id = unsafe { ptr::read(data.get_unchecked(pos)) };
        debug_assert!(all[frame_id as usize].1.is_some());
        Hole {
            data,
            all,
            elt: ManuallyDrop::new(frame_id),
            pos,
        }
    }

    /// Returns the position of the hole.
    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &LRUKNodeWrapper {
        self.all[*self.elt.deref() as usize].0.as_ref().unwrap()
    }

    /// Returns a reference to the element at `index`.
    ///
    /// # Safety
    ///
    /// Index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &LRUKNodeWrapper {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let frame_id = unsafe { self.data.get_unchecked(index) };
        self.all[*frame_id as usize].0.as_ref().unwrap()
    }

    /// Move hole to new location
    ///
    /// # Safety
    ///
    /// target_position must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, target_position: usize) {
        debug_assert!(target_position != self.pos);
        debug_assert!(target_position < self.data.len());
        unsafe {
            let ptr = self.data.as_mut_ptr();
            let target_ptr: *const _ = ptr.add(target_position);

            // update target index in key map
            let target_element = &*target_ptr;
            let old = self.all[*target_element as usize].1.replace(self.pos);

            old.expect(
                "Hole can only exist for key values pairs, that are already part of the heap.",
            );

            // move target into hole
            let hole_ptr = ptr.add(self.pos);
            ptr::copy_nonoverlapping(target_ptr, hole_ptr, 1);
        }
        // update hole position
        self.pos = target_position;
    }
}

impl Drop for Hole<'_> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::copy_nonoverlapping(&*self.elt, self.data.get_unchecked_mut(pos), 1);
        }
        let key = *self.elt.deref();
        let old = self.all[key as usize].1.replace(self.pos);

        old.expect(
            "Hole can only exist for key values pairs, that are already part of the heap.",
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::hash::Hash;
    use crate::buffer::replacer::lru_k_replacer::counter::AtomicI64Counter;
    use crate::buffer::replacer::lru_k_replacer::lru_k_node::LRUKNode;

    fn is_normal<T: Send + Unpin>() {}

    #[test]
    fn check_is_send_unpin() {
        is_normal::<LRUKReplacerStore>();
        assert!(true);
    }

    fn assert_key_map_valid(bh: &LRUKReplacerStore) {
        let mut expected_keys = HashMap::new();
        for (i, kv) in bh.data.iter().enumerate() {
            expected_keys.insert(kv.clone(), i);
        }

        for key_index in &expected_keys {
            let key = key_index.0;
            let index = *key_index.1;
            assert_eq!(bh.all[*key as usize].1, Some(index));
        }
        let keys_len = bh.all.iter().filter(|(_, item)| item.is_some()).count();
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
