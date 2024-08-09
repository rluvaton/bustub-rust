#[cfg(test)]
mod tests {
    use crate::lru_k_replacer::access_type::AccessType;
    use crate::lru_k_replacer::lru_k_replacer::LRUKReplacer;

    #[test]
    fn sample() {
        let mut lru_replacer: LRUKReplacer = LRUKReplacer::new(7, 2);

        // Scenario: add six elements to the replacer. We have [1,2,3,4,5]. Frame 6 is non-evictable.
        lru_replacer.record_access(1, AccessType::default());
        lru_replacer.record_access(2, AccessType::default());
        lru_replacer.record_access(3, AccessType::default());
        lru_replacer.record_access(4, AccessType::default());
        lru_replacer.record_access(5, AccessType::default());
        lru_replacer.record_access(6, AccessType::default());
        lru_replacer.set_evictable(1, true);
        lru_replacer.set_evictable(2, true);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        lru_replacer.set_evictable(5, true);
        lru_replacer.set_evictable(6, false);
        assert_eq!(lru_replacer.size(), 5);

        // Scenario: Insert access history for frame 1. Now frame 1 has two access histories.
        // All other frames have max backward k-dist. The order of eviction is [2,3,4,5,1].
        lru_replacer.record_access(1, AccessType::default());

        // Scenario: Evict three pages from the replacer. Elements with max k-distance should be popped
        // first based on LRU.
        assert_eq!(lru_replacer.evict(), Some(2));
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.evict(), Some(4));
        assert_eq!(lru_replacer.size(), 2);

        // Scenario: Now replacer has frames [5,1].
        // Insert new frames 3, 4, and update access history for 5. We should end with [3,1,5,4]
        lru_replacer.record_access(3, AccessType::default());
        lru_replacer.record_access(4, AccessType::default());
        lru_replacer.record_access(5, AccessType::default());
        lru_replacer.record_access(4, AccessType::default());
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        assert_eq!(lru_replacer.size(), 4);


        // Scenario: continue looking for victims. We expect 3 to be evicted next.
        assert_eq!(lru_replacer.evict(), Some(3));
        assert_eq!(lru_replacer.size(), 3);

        // Set 6 to be evictable. 6 Should be evicted next since it has max backward k-dist.
        lru_replacer.set_evictable(6, true);
        assert_eq!(lru_replacer.size(), 4);
        assert_eq!(lru_replacer.evict(), Some(6));
        assert_eq!(lru_replacer.size(), 3);

        // Now we have [1,5,4]. Continue looking for victims.
        lru_replacer.set_evictable(1, false);
        assert_eq!(lru_replacer.size(), 2);
        assert_eq!(lru_replacer.evict(), Some(5));
        assert_eq!(lru_replacer.size(), 1);

        // Update access history for 1. Now we have [4,1]. Next victim is 4.
        lru_replacer.record_access(1, AccessType::default());
        lru_replacer.record_access(1, AccessType::default());
        lru_replacer.set_evictable(1, true);
        assert_eq!(lru_replacer.size(), 2);
        assert_eq!(lru_replacer.evict(), Some(4));

        assert_eq!(lru_replacer.size(), 1);
        assert_eq!(lru_replacer.evict(), Some(1));
        assert_eq!(lru_replacer.size(), 0);

        // This operation should not modify size
        assert_eq!(lru_replacer.evict(), None);
        assert_eq!(lru_replacer.size(), 0);
    }

    // TODO - add tests for thread safety
}
