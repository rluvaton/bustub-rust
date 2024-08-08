use crate::trie_node_type::TrieNodeType;

#[derive(Clone, Debug)]
pub struct Trie {

    // The root of the trie.
    pub(crate) root: Option<TrieNodeType>,
}

impl Trie {

    // Create an empty trie.
    pub fn create_empty() -> Self {
        Trie {
            root: None,
        }
    }

    // Create a new trie with the given root.
    pub fn new(root: TrieNodeType) -> Self {
        Trie {
            root: Some(root),
        }
    }
}
