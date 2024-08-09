use std::rc::Rc;
use crate::Trie;

// This class is a thread-safe wrapper around the Trie class. It provides a simple interface for
// accessing the trie. It should allow concurrent reads and a single write operation at the same
// time.
#[derive(Clone, Debug)]
pub struct TrieStore {

    // Stores the current root for the trie.
    pub(crate) root: Trie,
}

impl<'a> TrieStore {

    // Create an empty trie.
    pub fn create_empty() -> Rc<Self> {
        Rc::new(Trie {
            root: None,
        })
    }

    // Create a new trie with the given root.
    pub fn new(root: Rc<TrieNode>) -> Rc<Self> {
        Rc::new(Trie {
            root: Some(Rc::clone(&root)),
        })
    }
}
