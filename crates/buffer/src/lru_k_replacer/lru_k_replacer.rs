use crate::lru_k_replacer::access_type::AccessType;
use common::config::FrameId;
use std::sync::{Arc, Mutex};
use crate::lru_k_replacer::LRUKReplacerImpl;

// Cloning does not actually clone the underlying data but just increment the ref count
#[derive(Debug, Clone)]
pub struct LRUKReplacer {
    // This lock every call instead of smarter lock, or when can lock smaller parts of the code
    pub(super) replacer: Arc<Mutex<LRUKReplacerImpl>>,
}


// Proxy to LRU-K Replacer Impl
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
    pub fn new(num_frames: usize, k: usize) -> Self {
        LRUKReplacer {
            replacer: Arc::new(Mutex::new(LRUKReplacerImpl::new(num_frames, k)))
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
        self.replacer.lock().unwrap().evict()
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
        self.replacer.lock().unwrap().record_access(frame_id, access_type)
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
        self.replacer.lock().unwrap().set_evictable(frame_id, set_evictable)
    }

    pub fn is_evictable(&self, frame_id: FrameId) -> Option<bool> {
        self.replacer.lock().unwrap().is_evictable(frame_id)
    }

    pub unsafe fn is_evictable_unchecked(&self, frame_id: FrameId) -> bool {
        self.replacer.lock().unwrap().is_evictable_unchecked(frame_id)
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
        self.replacer.lock().unwrap().remove(frame_id)
    }

    /// Replacer's size, which tracks the number of evictable frames.
    ///
    /// returns: isize the number of evictable frames
    ///
    pub fn size(&self) -> usize {
        self.replacer.lock().unwrap().size()
    }

    pub(super) fn get_order_of_eviction(&self) -> Vec<FrameId> {
        self.replacer.lock().unwrap().get_order_of_eviction()
    }
}

