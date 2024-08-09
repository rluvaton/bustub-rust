use std::collections::HashMap;
use common::config::FrameId;
use crate::lru_k_replacer::access_type::AccessType;
use crate::lru_k_replacer::lru_k_replacer::LRUKReplacer;

impl LRUKReplacer {
    /// a new `LRUKReplacer`
    ///
    /// # Arguments
    ///
    /// * `num_frames`: the maximum number of frames the LRUReplacer will be required to store
    /// * `k`: the `k` in the LRU-K
    ///
    /// returns: LRUKReplacer
    ///
    pub fn new(num_frames: isize, k: isize) -> Self {
        LRUKReplacer {
            node_store: HashMap::with_capacity(num_frames as usize),
            k,

            // Default
            current_timestamp: 0,

            // let not really needed?
            curr_size: 0,

            // Not really needed
            replacer_size: num_frames,
            latch: None,


            evictable_frames: 0,
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
        unimplemented!()
    }

    /// Record the event that the given frame id is accessed at current timestamp.
    /// Create a new entry for access history if frame id has not been seen before.
    ///
    /// If frame id is invalid (i.e. larger than replacer_size_), throw an exception. You can
    /// also use BUSTUB_ASSERT to abort the process if frame id is invalid.
    ///
    /// If a frame was previously evictable and is to be set to non-evictable, then size should
    /// decrement. If a frame was previously non-evictable and is to be set to evictable,
    /// then size should increment.
    ///
    /// If frame is missing or invalid, this function should terminate without modifying anything.
    ///
    /// # Arguments
    ///
    /// * `frame_id`: id of frame that received a new access.
    /// * `access_type`: type of access that was received.
    ///                  This parameter is only needed for leaderboard tests.
    ///
    pub fn record_access(&mut self, frame_id: FrameId, access_type: AccessType) {
        // TODO = default access type is unknown
        unimplemented!()
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

        // Nothing to change
        if node.is_evictable == set_evictable {
            return
        }

        // If evictable, mark as no longer evictable or vice versa
        self.evictable_frames += if node.is_evictable { -1 } else { 1 };

        node.is_evictable = set_evictable;
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
        if frame.is_none() || !frame.unwrap().is_evictable {
            return
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
    pub unsafe fn remove_unchecked(&mut self, frame_id: FrameId) {
        let frame = self.node_store.get(&frame_id);
        if frame.is_none() {
            return
        }

        let frame = frame.unwrap();

        assert_eq!(frame.is_evictable, true, "Frame must be evictable");

        // If somehow deleted in the middle only decrease in case actually removed
        if self.node_store.remove(&frame_id).is_some() {
            // Decrease evictable frames
            self.evictable_frames -= 1;
        }
    }

    /// Replacer's size, which tracks the number of evictable frames.
    ///
    /// returns: isize the number of evictable frames
    ///
    pub fn size(&self) -> isize {
        self.evictable_frames as isize
    }

    fn is_valid_frame_id(&self, frame_id: FrameId) -> bool {
        self.node_store.capacity() < frame_id as usize
    }

    fn assert_valid_frame_id(&self, frame_id: FrameId) {
        assert!(self.is_valid_frame_id(frame_id));
    }
}
