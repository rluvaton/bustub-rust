use super::AtomicI64Counter;

/// This is
pub(super) trait LRUNode {

    fn reuse(&mut self, counter: &AtomicI64Counter);

    fn marked_accessed(&mut self, counter: &AtomicI64Counter);

    #[inline]
    fn is_evictable(&self) -> bool;

    #[inline]
    fn set_evictable(&mut self, evictable: bool);
}
