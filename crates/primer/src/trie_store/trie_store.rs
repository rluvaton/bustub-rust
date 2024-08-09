use crate::Trie;
use std::sync::{Arc, Mutex};

// This class is a thread-safe wrapper around the Trie class. It provides a simple interface for
// accessing the trie. It should allow concurrent reads and a single write operation at the same
// time.
#[derive(Clone, Debug)]
pub struct TrieStore {

    // Stores the current root for the trie.
    pub(crate) root: Arc<Trie>,

    // This mutex protects the root. Every time you want to access the trie root or modify it, you
    // will need to take this lock.
    pub(crate) root_lock: Arc<Mutex<Arc<Trie>>>,

    // This mutex sequences all writes operations and allows only one write operation at a time.
    pub(crate) write_lock: Arc<Mutex<Arc<Trie>>>,
}

impl TrieStore {

    pub fn new() -> Self {

        let trie = Trie::create_empty();

        TrieStore {
            root: Arc::clone(&trie),
            root_lock: Arc::new(Mutex::new(Arc::clone(&trie))),
            write_lock: Arc::new(Mutex::new(Arc::clone(&trie))),
        }
    }
}
