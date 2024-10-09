use std::hash::Hash;
use std::sync::Arc;
#[cfg(feature = "tracing")]
use tracy_client::span;

use super::counter::AtomicI64Counter;
use super::store::Store;
use buffer_common::{AccessType, FrameId};
use crate::EvictionPolicy;
use crate::lru_k::LRUKOptions;
use crate::traits::EvictionPolicyCreator;

/**
 * LRUKEvictionPolicy implements the LRU-k replacement policy.
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
pub struct LRUKEvictionPolicy {
    store: Store,
    replacer_size: usize,

    // Tracks the number of evictable frames
    evictable_frames: usize,

    history_access_counter: Arc<AtomicI64Counter>,
}

unsafe impl Send for LRUKEvictionPolicy {}

impl LRUKEvictionPolicy {

    fn is_valid_frame_id(&self, frame_id: FrameId) -> bool {
        self.replacer_size > frame_id as usize
    }

    fn assert_valid_frame_id(&self, frame_id: FrameId) {
        assert!(self.is_valid_frame_id(frame_id));
    }

    /// Helper for debugging in tests
    pub(crate) fn get_order_of_eviction(&self) -> Vec<FrameId> {
        self.store.clone().get_order_of_eviction()
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
    unsafe fn record_access_unchecked(&mut self, frame_id: FrameId, _access_type: AccessType) {
        let mut node = self.store.get_node(frame_id);
        let node = node.as_mut();

        if node.is_none() {
            // Not inserting to evictable frames as new frame is not evictable by default
            self.store.add_non_evictable_node(
                frame_id,
                &self.history_access_counter,
            );

            return;
        }

        let node = node.unwrap();

        node.marked_accessed(&self.history_access_counter);

        // if evictable, the node should be reinserted as it's location would be updated
        if node.is_evictable() {
            // Update the heap with the updated value
            self.store.update_after_evictable(frame_id);
        }
    }
}

impl EvictionPolicyCreator for LRUKEvictionPolicy {
    type Options = LRUKOptions;

    /// a new `LRUKEvictionPolicy`
    ///
    /// # Arguments
    ///
    /// * `num_frames`: the maximum number of frames the LRUReplacer will be required to store
    /// * `k`: the `k` in the LRU-K
    ///
    /// returns: LRUKEvictionPolicy
    ///
    fn new(num_frames: usize, options: LRUKOptions) -> Self {
        Self {
            store: Store::with_capacity(options.k, num_frames),
            replacer_size: num_frames,
            evictable_frames: 0,
            history_access_counter: Arc::new(AtomicI64Counter::default()),
        }
    }
}

impl EvictionPolicy for LRUKEvictionPolicy {
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
    fn evict(&mut self) -> Option<FrameId> {
        #[cfg(feature = "tracing")]
        let _evict = span!("Evict");

        let frame_id = self.store.remove_next_evictable_frame()?;

        // Decrease evictable frames
        self.evictable_frames -= 1;

        Some(frame_id)
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
    fn record_access(&mut self, frame_id: FrameId, access_type: AccessType) {
        if !self.is_valid_frame_id(frame_id) {
            return;
        }

        unsafe {
            self.record_access_unchecked(frame_id, access_type);
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
    fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) {
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
    unsafe fn set_evictable_unchecked(&mut self, frame_id: FrameId, set_evictable: bool) {
        // We are certain that the frame id is valid
        self.assert_valid_frame_id(frame_id);

        // if missing - nothing to do, if evictable status is the same - Nothing to change
        if !self.store.has_node(frame_id) || self.store.is_existing_node_evictable_unchecked(frame_id) == set_evictable {
            return;
        }

        // If about to be evictable, mark it and update the count
        if set_evictable {
            self.evictable_frames += 1;
            self.store.mark_frame_as_evictable(frame_id);
        } else {
            self.evictable_frames -= 1;
            self.store.mark_frame_as_not_evictable(&frame_id);
        }
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
    fn remove(&mut self, frame_id: FrameId) {
        let removed = self.store.remove_node_if_evictable(frame_id);

        // Decrease evictable frames
        self.evictable_frames -= removed as usize;
    }

    /// Replacer's size, which tracks the number of evictable frames.
    ///
    /// returns: isize the number of evictable frames
    ///
    fn size(&self) -> usize {
        self.evictable_frames
    }
}
