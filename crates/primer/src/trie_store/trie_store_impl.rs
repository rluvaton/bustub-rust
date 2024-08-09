// This file is what the students will be implementing

use std::collections::HashMap;
use std::rc::Rc;
use crate::trie::trie::Trie;
use crate::trie::trie_node::TrieNode;
use crate::trie::trie_node_value_types::TrieNodeValueTypes;
use crate::trie_store::trie_store::TrieStore;

enum RemoveResult {
    NotFound,

    // If node has children
    ReplaceNode(Rc<TrieNode>),

    // Remove node completely, if the found node has no children
    Remove,
}

impl Trie {
    // This function returns a ValueGuard object that holds a reference to the value in the trie. If
    // the key does not exist in the trie, it will return std::nullopt.
    pub fn get(self: &TrieStore, key: &str) -> Option<&TrieNodeValueTypes> {

        // Pseudo-code:
        // (1) Take the root lock, get the root, and release the root lock. Don't lookup the value in the
        //     trie while holding the root lock.
        // (2) Lookup the value in the trie.
        // (3) If the value is found, return a ValueGuard object that holds a reference to the value and the
        //     root. Otherwise, return std::nullopt.
        unimplemented!()
    }

    // This function will insert the key-value pair into the trie. If the key already exists in the
    // trie, it will overwrite the value.
    pub fn put(&self, key: &str, value: TrieNodeValueTypes) {
        // You will need to ensure there is only one writer at a time. Think of how you can achieve this.
        // The logic should be somehow similar to `TrieStore::Get`.
        unimplemented!()
    }

    // This function will remove the key-value pair from the trie.
    pub fn remove(&self, key: &str) {
        // You will need to ensure there is only one writer at a time. Think of how you can achieve this.
        // The logic should be somehow similar to `TrieStore::Get`.
        unimplemented!()
    }
}
