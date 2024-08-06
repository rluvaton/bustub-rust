// TODO - implement MoveBlocked?

use std::collections::HashMap;
use crate::trie_node_type::TrieNodeType;

#[derive(Clone, Debug)]
pub struct TrieNode<T: Clone> {

    // A map of children, where the key is the next character in the key, and the value is the next TrieNode.
    // You MUST store the children information in this structure. You are NOT allowed to remove the `const` from
    // the structure.
    children: Option<HashMap<char, TrieNodeType<T>>>,

    // Indicates if the node is the terminal node.
    // is_value_node: bool
}

impl<T: Clone> TrieNode<T> {

    // Create a TrieNode with no children.
    fn empty() -> Self {
        TrieNode {
            children: None,
        }
    }

    fn new(children: HashMap<char, TrieNodeType<T>>) -> Self {
        TrieNode {
            children: Some(children),
        }
    }
}
