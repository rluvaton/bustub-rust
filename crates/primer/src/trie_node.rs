// TODO - implement MoveBlocked?

use std::collections::HashMap;
use std::rc::Rc;
use crate::trie_node_type::TrieNodeType;
use crate::trie_node_with_value::TrieNodeWithValue;

#[derive(Clone, Debug, PartialEq)]
pub struct TrieNode {

    // A map of children, where the key is the next character in the key, and the value is the next TrieNode.
    // You MUST store the children information in this structure. You are NOT allowed to remove the `const` from
    // the structure.

    // TODO - made the value be Cow
    pub(crate) children: Option<HashMap<char, Rc<TrieNodeType>>>,

    // Indicates if the node is the terminal node.
    // is_value_node: bool
}

impl TrieNode {

    // Create a TrieNode with no children.
    pub fn empty() -> Self {
        TrieNode {
            children: None,
        }
    }

    pub fn new(children: HashMap<char, Rc<TrieNodeType>>) -> Self {
        TrieNode {
            children: Some(children),
        }
    }
}

impl From<TrieNodeWithValue> for TrieNode {
    fn from(value: TrieNodeWithValue) -> Self {
        TrieNode {
            children: value.children
        }
    }
}


impl From<&TrieNodeWithValue> for TrieNode {
    fn from(value: &TrieNodeWithValue) -> Self {
        TrieNode {
            children: value.children.clone()
        }
    }
}
