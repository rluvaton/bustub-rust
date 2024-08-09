use crate::trie::trie_node::TrieNode;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Trie {

    // The root of the trie.
    pub(crate) root: Option<Arc<TrieNode>>,
}

impl Trie {

    // Create an empty trie.
    pub fn create_empty() -> Arc<Self> {
        Arc::new(Trie {
            root: None,
        })
    }

    // Create a new trie with the given root.
    pub fn new(root: Arc<TrieNode>) -> Arc<Self> {
        Arc::new(Trie {
            root: Some(Arc::clone(&root)),
        })
    }
}
