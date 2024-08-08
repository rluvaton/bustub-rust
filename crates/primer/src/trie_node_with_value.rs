use std::collections::HashMap;
use crate::trie_node_type::TrieNodeType;
use crate::trie_node_value_types::TrieNodeValueTypes;

#[derive(Clone, Debug, PartialEq)]
pub struct TrieNodeWithValue {

    pub(crate) value: TrieNodeValueTypes,

    // A map of children, where the key is the next character in the key, and the value is the next TrieNode.
    // You MUST store the children information in this structure. You are NOT allowed to remove the `const` from
    // the structure.

    // TODO - made the value be Cow
    pub(crate) children: Option<HashMap<char, TrieNodeType>>,

    // Indicates if the node is the terminal node.
    // is_value_node: bool
}

impl TrieNodeWithValue {

    pub fn new(children: Option<HashMap<char, TrieNodeType>>, value: TrieNodeValueTypes) -> Self {
        TrieNodeWithValue {
            value,
            children,
        }
    }

    // Create a TrieNode with no children.
    pub fn with_value(value: TrieNodeValueTypes) -> Self {
        TrieNodeWithValue {
            value,
            children: None,
        }
    }
}
