use crate::trie_node_type::TrieNodeType;

#[derive(Clone, Debug)]
pub struct Trie<T: Clone + Sized> {

    // The root of the trie.
    root: Option<TrieNodeType<T>>,
}

impl<T: Clone + Sized> Trie<T> {

    // Create an empty trie.
    pub fn create_empty() -> Self {
        Trie {
            root: None,
        }
    }

    // Create a new trie with the given root.
    pub fn new(root: TrieNodeType<T>) -> Self {
        Trie {
            root: Some(root),
        }
    }
}
