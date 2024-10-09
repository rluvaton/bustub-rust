
/// lookback window for lru-k replacer
pub const LRUK_REPLACER_K: usize = 10;

pub struct LRUKOptions {
    pub(super) k: usize,
}

impl LRUKOptions {
    pub fn new(k: usize) -> Self {
        Self {
            k
        }
    }
}

impl Default for LRUKOptions {
    fn default() -> Self {
        Self {
            k: LRUK_REPLACER_K
        }
    }
}
