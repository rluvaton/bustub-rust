// This file is what the students will be implementing

use crate::trie::trie_node_value_types::TrieNodeValueTypes;
use crate::trie_store::trie_store::TrieStore;
use crate::Trie;
use std::sync::{Arc, MutexGuard};

impl TrieStore {
    // This function returns a ValueGuard object that holds a reference to the value in the trie. If
    // the key does not exist in the trie, it will return std::nullopt.
    pub fn get(&self, key: &str) -> Option<TrieNodeValueTypes> {

        let root: Arc<Trie>;
        // Pseudo-code:
        // (1) Take the root lock, get the root, and release the root lock. Don't lookup the value in the
        //     trie while holding the root lock.
        {
            let guard = self.root_lock.lock().unwrap();
            root = Arc::clone(&guard);

            // Lock will be released here when the closure ends
        }

        // (2) Lookup the value in the trie.
        let val = root.get(key);


        // (3) If the value is found, return a ValueGuard object that holds a reference to the value and the
        //     root. Otherwise, return std::nullopt.
        // TODO - add value guard?
        val.cloned()
    }

    // This function will insert the key-value pair into the trie. If the key already exists in the
    // trie, it will overwrite the value.
    pub fn put(&mut self, key: &str, value: TrieNodeValueTypes) {
        // You will need to ensure there is only one writer at a time. Think of how you can achieve this.
        // The logic should be somehow similar to `TrieStore::Get`.

        // After got the root, lock in the write lock to avoid multiple writers
        let mut write_guard = self.write_lock.lock().unwrap();

        // create new trie
        let new_root = write_guard.put(key, value);

        // ###### Update root

        // Lock again the root so we can modify it
        let mut guard = self.root_lock.lock().unwrap();

        *guard = Arc::clone(&new_root);
        *write_guard = Arc::clone(&new_root);
        self.root = Arc::clone(&new_root)

        // Release the write lock
    }

    // This function will remove the key-value pair from the trie.
    pub fn remove(&mut self, key: &str) {
        // You will need to ensure there is only one writer at a time. Think of how you can achieve this.
        // The logic should be somehow similar to `TrieStore::Get`.


        // After got the root, lock in the write lock to avoid multiple writers
        let mut write_guard = self.write_lock.lock().unwrap();

        // create new trie
        let new_root = write_guard.remove(key);

        // ###### Update root
        // Lock again the root so we can modify it
        let mut guard = self.root_lock.lock().unwrap();

        *guard = Arc::clone(&new_root);
        *write_guard = Arc::clone(&new_root);
        self.root = Arc::clone(&new_root)

        // Release the write lock
    }

    // TODO - fix this function error
    fn update_root(&mut self, new_root: &Arc<Trie>, mut write_guard: MutexGuard<Arc<Trie>>) {
        // Lock again the root so we can modify it
        let mut guard = self.root_lock.lock().unwrap();

        *guard = Arc::clone(&new_root);
        *write_guard = Arc::clone(&new_root);
        self.root = Arc::clone(&new_root)
    }
}
