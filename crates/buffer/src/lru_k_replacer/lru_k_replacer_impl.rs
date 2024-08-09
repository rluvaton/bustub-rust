use common::config::FrameId;
use crate::lru_k_replacer::access_type::AccessType;
use crate::lru_k_replacer::lru_k_replacer::LRUKReplacer;

impl LRUKReplacer {

    /**
     *
     * TODO(P1): Add implementation
     *
     * @brief a new LRUKReplacer.
     * @param num_frames the maximum number of frames the LRUReplacer will be required to store
     */
    pub fn new(num_frames: isize, k: isize) -> Self {
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Find the frame with largest backward k-distance and evict that frame. Only frames
     * that are marked as 'evictable' are candidates for eviction.
     *
     * A frame with less than k historical references is given +inf as its backward k-distance.
     * If multiple frames have inf backward k-distance, then evict frame with earliest timestamp
     * based on LRU.
     *
     * Successful eviction of a frame should decrement the size of replacer and remove the frame's
     * access history.
     *
     * @param[out] frame_id id of frame that is evicted.
     * @return true if a frame is evicted successfully, false if no frames can be evicted.
     *
     * Returns None if non frames can be evicted or Some(FrameId) of the successful frame that evicted
     */

    pub fn evict(&mut self) -> Option<FrameId> {
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Record the event that the given frame id is accessed at current timestamp.
     * Create a new entry for access history if frame id has not been seen before.
     *
     * If frame id is invalid (ie. larger than replacer_size_), throw an exception. You can
     * also use BUSTUB_ASSERT to abort the process if frame id is invalid.
     *
     * @param frame_id id of frame that received a new access.
     * @param access_type type of access that was received. This parameter is only needed for
     * leaderboard tests.
     */
    pub fn record_access(&mut self, frame_id: FrameId, access_type: AccessType) {
        // TODO = default access type is unknown
        unimplemented!()
    }


    /**
     * TODO(P1): Add implementation
     *
     * @brief Toggle whether a frame is evictable or non-evictable. This function also
     * controls replacer's size. Note that size is equal to number of evictable entries.
     *
     * If a frame was previously evictable and is to be set to non-evictable, then size should
     * decrement. If a frame was previously non-evictable and is to be set to evictable,
     * then size should increment.
     *
     * If frame id is invalid, throw an exception or abort the process.
     *
     * For other scenarios, this function should terminate without modifying anything.
     *
     * @param frame_id id of frame whose 'evictable' status will be modified
     * @param set_evictable whether the given frame is evictable or not
     */
    pub fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) {
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Remove an evictable frame from replacer, along with its access history.
     * This function should also decrement replacer's size if removal is successful.
     *
     * Note that this is different from evicting a frame, which always remove the frame
     * with largest backward k-distance. This function removes specified frame id,
     * no matter what its backward k-distance is.
     *
     * If Remove is called on a non-evictable frame, throw an exception or abort the
     * process.
     *
     * If specified frame is not found, directly return from this function.
     *
     * @param frame_id id of frame to be removed
     */
    pub fn remove(&mut self, frame_id: FrameId) {
        unimplemented!()
    }

    /**
     * TODO(P1): Add implementation
     *
     * @brief Return replacer's size, which tracks the number of evictable frames.
     *
     * @return size_t
     */
    pub fn size(&self) -> isize {
        unimplemented!()
    }

}
