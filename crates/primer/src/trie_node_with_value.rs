use std::collections::HashMap;
use crate::trie_node::TrieNode;
use crate::trie_node_type::TrieNodeType;

#[derive(Clone, Debug)]
pub struct TrieNodeWithValue<T: Clone> {

    pub(crate) value: T,

    // A map of children, where the key is the next character in the key, and the value is the next TrieNode.
    // You MUST store the children information in this structure. You are NOT allowed to remove the `const` from
    // the structure.
    pub(crate) children: Option<HashMap<char, TrieNodeType<T>>>,

    // Indicates if the node is the terminal node.
    // is_value_node: bool
}

impl<T: Clone> TrieNodeWithValue<T> {

    pub fn new(children: Option<HashMap<char, TrieNodeType<T>>>, value: T) -> Self {
        TrieNodeWithValue {
            value,
            children,
        }
    }

    // Create a TrieNode with no children.
    pub fn with_value(value: T) -> Self {
        TrieNodeWithValue {
            value,
            children: None,
        }
    }
}
