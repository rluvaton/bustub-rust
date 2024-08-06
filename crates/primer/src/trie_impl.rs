// This file is what the students will be implementing

use crate::trie::Trie;

impl<V: Clone + Sized> Trie<V> {

    // Get the value associated with the given key.
    // 1. If the key is not in the trie, return nullptr.
    // 2. If the key is in the trie but the type is mismatched, return nullptr.
    // 3. Otherwise, return the value.
    pub fn get<T: Clone + Sized>(&self, key: &str) -> T {
        unimplemented!()
    }

    // Put a new key-value pair into the trie. If the key already exists, overwrite the value.
    // Returns the new trie.
    pub fn put<T: Clone + Sized>(&self, key: &str, value: T) -> Self {
        unimplemented!()
    }

    // Remove the key from the trie. If the key does not exist, return the original trie.
    // Otherwise, returns the new trie.
    pub fn remove(&self, key: &str) -> Self {
        unimplemented!()
    }
}
