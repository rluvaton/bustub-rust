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
    pub fn new(num_frames: usize, k: usize) -> Self {
        LRUKReplacer {
            replacer: Arc::new(Mutex::new(LRUKReplacerImpl::new(num_frames, k)))
        }
    }

    pub fn evict(&mut self) -> Option<FrameId> {
        self.replacer.lock().unwrap().evict()
    }

    pub fn record_access(&mut self, frame_id: FrameId, access_type: AccessType) {
        self.replacer.lock().unwrap().record_access(frame_id, access_type)
    }

    pub fn set_evictable(&mut self, frame_id: FrameId, set_evictable: bool) {
        self.replacer.lock().unwrap().set_evictable(frame_id, set_evictable)
    }

    pub fn remove(&mut self, frame_id: FrameId) {
        self.replacer.lock().unwrap().remove(frame_id)
    }

    pub fn size(&self) -> usize {
        self.replacer.lock().unwrap().size()
    }

    pub(super) fn get_order_of_eviction(&self) -> Vec<FrameId> {
        self.replacer.lock().unwrap().get_order_of_eviction()
    }
}

