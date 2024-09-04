use mut_binary_heap::{BinaryHeap, FnComparator};
use std::cell::UnsafeCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;
use tracy_client::span;

use common::config::FrameId;

use super::access_type::AccessType;
use super::counter::AtomicU64Counter;
use super::lru_k_node::LRUKNode;

type LRUKNodeWrapper = Arc<UnsafeCell<LRUKNode>>;

/**
 * LRUKReplacer implements the LRU-k replacement policy.
 *
 * The LRU-k algorithm evicts a frame whose backward k-distance is maximum
 * of all frames. Backward k-distance is computed as the difference in time between
 * current timestamp and the timestamp of kth previous access.
 *
 * A frame with less than k historical references is given
 * +inf as its backward k-distance. When multiple frames have +inf backward k-distance,
 * classical LRU algorithm is used to choose victim.
 */
#[derive(Clone, Debug)]
pub struct LRUKReplacerImpl {
    /// in cpp it was unordered_map
    node_store: HashMap<FrameId, LRUKNodeWrapper>,

    /// Heap for evictable LRU-K nodes for best performance for finding evictable frames
    /// This is mutable Heap to allow for updating LRU-K Node without removing and reinserting
    evictable_heap: BinaryHeap<FrameId, LRUKNodeWrapper, FnComparator<fn(&LRUKNodeWrapper, &LRUKNodeWrapper) -> Ordering>>,

    replacer_size: usize,

    k: usize,

    // Tracks the number of evictable frames
    evictable_frames: usize,

    history_access_counter: Arc<AtomicU64Counter>,
}

unsafe impl Send for LRUKReplacerImpl {}


impl LRUKReplacerImpl {
    /// a new `LRUKReplacerImpl`
    ///
    /// # Arguments
    ///
    /// * `num_frames`: the maximum number of frames the LRUReplacer will be required to store
    /// * `k`: the `k` in the LRU-K
    ///
    /// returns: LRUKReplacerImpl
    ///
    pub(super) fn new(num_frames: usize, k: usize) -> Self {
        LRUKReplacerImpl {
            node_store: HashMap::with_capacity(num_frames),
            evictable_heap: BinaryHeap::with_capacity_by(num_frames, |a, b| {
                unsafe {
                    (*a.get()).cmp(&*b.get())
                }
            }),
            k,

            replacer_size: num_frames,

            evictable_frames: 0,

            history_access_counter: Arc::new(AtomicU64Counter::default()),
        }
    }

    /// Find the frame with the largest backward k-distance and evict that frame. Only frames
    /// that are marked as `evictable` are candidates for eviction.
    ///
    /// A frame with less than k historical references is given +inf as its backward k-distance.
    /// If multiple frames have inf backward k-distance, then evict frame with the earliest timestamp
    /// based on LRU.
    ///
    /// Successful eviction of a frame should decrement the size of replacer and remove the frame's
    /// access history.
    ///
    /// returns: Option<FrameId> `None` if no frame to evict or `Some(FrameId)` with the frame that
    ///          got evicted
    ///
    pub fn evict(&mut self) -> Option<FrameId> {
        let _unpin = span!("Evict");

        // No frame is evictable
        if self.size() == 0 {
            return None;
        }

        // This is not really performant, and we should better track the most likely to be evicted in a sorted way
        let evictable_frame = self
            .get_next_to_evict()
            .expect("Cant have missing evictable frame");

        // We know for sure that the frame is evictable
        unsafe { self.remove_unchecked(evictable_frame) }

        Some(evictable_frame)
    }

    /// Record the event that the given frame id is accessed at current timestamp.
    /// Create a new entry for access history if frame id has not been seen before.
    ///
    /// If frame id is invalid nothing is done
    ///
    /// # Arguments
    ///
    /// * `frame_id`: id of frame that received a new access.
    /// * `access_type`: type of access that was received.
    ///                  This parameter is only needed for leaderboard tests.
    ///
    pub fn record_access(&mut self, frame_id: FrameId, access_type: AccessType) {
        if !self.is_valid_frame_id(frame_id) {
            return;
        }

        unsafe {
            self.record_access_unchecked(frame_id, access_type);
        }
    }

    /// Record the event that the given frame id is accessed at current timestamp.
    /// Create a new entry for access history if frame id has not been seen before.
    ///
    /// If frame id is invalid (i.e. larger than replacer_size_), throw an exception
    ///
    /// # Arguments
    ///
    /// * `frame_id`: id of frame that received a new access.
    /// * `access_type`: type of access that was received.
    ///                  This parameter is only needed for leaderboard tests.
    ///
    /// # Unsafe
    /// unsafe as we are certain that the frame id is valid
    pub unsafe fn record_access_unchecked(&mut self, frame_id: FrameId, _access_type: AccessType) {
        self.assert_valid_frame_id(frame_id);

        let node = self.node_store.get_mut(&frame_id);

        if node.is_none() {
            let node = Arc::new(UnsafeCell::new(LRUKNode::new(self.k, frame_id, &self.history_access_counter)));
            self.node_store.insert(frame_id, node);

            // Not inserting to evictable frames as new frame is not evictable by default
            return;
        }

        let node = node.unwrap();

        unsafe {
            let inner = node.get();
            (*inner).marked_accessed(&self.history_access_counter);

            // if evictable, the node should be reinserted as it's location would be updated
            if (*inner).is_evictable() {
                // Update the heap with the updated value
                self.evictable_heap.push(frame_id, Arc::clone(&node));
            }
        }
    }

    /// Toggle whether a frame is evictable or non-evictable. This function also
    /// controls replacer's size. Note that size is equal to number of evictable entries.
    ///
    /// If a frame was previously evictable and is to be set to non-evictable, then size should
    /// decrement. If a frame was previously non-evictable and is to be set to evictable,
    /// then size should increment.
    ///
    /// If frame is missing or invalid, this function should terminate without modifying anything.
    ///
    /// # Arguments
    ///
    /// * `frame_id`: id of the frame whose `evictable` status will be modified
    /// * `set_evictable`: whether the given frame should be evictable or not
    ///
    pub fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) {
        if !self.is_valid_frame_id(frame_id) {
            return;
        }

        unsafe { self.set_evictable_unchecked(frame_id, set_evictable) }
    }

    /// Toggle whether a frame is evictable or non-evictable. This function also
    /// controls replacer's size. Note that size is equal to number of evictable entries.
    ///
    /// If a frame was previously evictable and is to be set to non-evictable, then size should
    /// decrement. If a frame was previously non-evictable and is to be set to evictable,
    /// then size should increment.
    ///
    /// If frame id is invalid, throw an exception or abort the process.
    ///
    /// For other scenarios, this function should terminate without modifying anything.
    ///
    /// # Arguments
    ///
    /// * `frame_id`: id of the frame whose `evictable` status will be modified
    /// * `set_evictable`: whether the given frame should be evictable or not
    ///
    /// # Unsafe
    /// This is unsafe because we are certain that the frame id is valid
    pub unsafe fn set_evictable_unchecked(&mut self, frame_id: FrameId, set_evictable: bool) {
        // We are certain that the frame id is valid
        self.assert_valid_frame_id(frame_id);

        let node = self.node_store.get_mut(&frame_id);

        // Nothing to do
        if node.is_none() {
            return;
        }

        let node = node.unwrap();

        let node_inner = node.get();

        // Nothing to change
        if (*node_inner).is_evictable() == set_evictable {
            return;
        }

        // If evictable, mark as no longer evictable or vice versa
        if (*node_inner).is_evictable() {
            self.evictable_frames -= 1;

            self.evictable_heap.remove(&frame_id);
        } else {
            self.evictable_frames += 1;

            self.evictable_heap.push(frame_id, Arc::clone(&node));
        }

        (*node_inner).set_evictable(set_evictable);
    }

    /// Remove an evictable frame from replacer, along with its access history.
    /// This function should also decrement replacer's size if removal is successful.
    ///
    /// Note that this is different from evicting a frame, which always remove the frame
    /// with the largest backward k-distance. This function removes specified frame id,
    /// no matter what its backward k-distance is.
    ///
    /// If specified frame is not found, or the frame is non-evictable, nothing is done.
    ///
    /// # Arguments
    ///
    /// * `frame_id`: Frame ID to remove, the frame must be evictable
    ///
    pub fn remove(&mut self, frame_id: FrameId) {
        let frame = self.node_store.get(&frame_id);

        unsafe {
            if frame.is_none() || !(*frame.unwrap().get()).is_evictable() {
                return;
            }
        }

        unsafe {
            self.remove_unchecked(frame_id);
        }
    }

    /// Remove an evictable frame from replacer, along with its access history.
    /// This function should also decrement replacer's size if removal is successful.
    ///
    /// Note that this is different from evicting a frame, which always remove the frame
    /// with the largest backward k-distance. This function removes specified frame id,
    /// no matter what its backward k-distance is.
    ///
    /// If `remove` is called on a non-evictable frame, throw an exception or abort the
    /// process.
    ///
    /// # Arguments
    ///
    /// * `frame_id`: Frame ID to remove, the frame must be evictable
    ///
    /// # Unsafe:
    /// This is unsafe because we are certain that the frame is evictable
    ///
    /// # Visibility
    /// function is not public as it is not locking by itself
    /// as the functions that called it should be responsible for that
    ///
    unsafe fn remove_unchecked(&mut self, frame_id: FrameId) {
        let frame = self.node_store.get(&frame_id);
        if frame.is_none() {
            return;
        }

        let frame = frame.unwrap();

        assert_eq!((*frame.get()).is_evictable(), true, "Frame must be evictable");

        // If somehow deleted in the middle only decrease in case actually removed
        if self.node_store.remove(&frame_id).is_some() {
            // Decrease evictable frames
            self.evictable_frames -= 1;
        }

        self.evictable_heap.remove(&frame_id);
    }

    /// Replacer's size, which tracks the number of evictable frames.
    ///
    /// returns: isize the number of evictable frames
    ///
    pub fn size(&self) -> usize {
        self.evictable_frames
    }

    fn is_valid_frame_id(&self, frame_id: FrameId) -> bool {
        self.replacer_size > frame_id as usize
    }

    fn assert_valid_frame_id(&self, frame_id: FrameId) {
        assert!(self.is_valid_frame_id(frame_id));
    }

    /// Helper for debugging in tests
    pub(in crate::buffer) fn get_order_of_eviction(&self) -> Vec<FrameId> {
        unsafe {
            self.evictable_heap.clone().into_iter_sorted().map(|item| (*item.get()).get_frame_id()).collect()
        }
    }

    fn get_next_to_evict(&mut self) -> Option<FrameId> {
        let top = self.evictable_heap.peek()?;

        unsafe { Some((*top.get()).get_frame_id()) }
    }
}

