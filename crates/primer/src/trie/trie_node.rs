
use crate::trie::trie_node_value_types::TrieNodeValueTypes;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct TrieNode {

    // A map of children, where the key is the next character in the key, and the value is the next TrieNode.
    // You MUST store the children information in this structure. You are NOT allowed to remove the `const` from
    // the structure.
    pub(crate) children: Option<HashMap<char, Arc<TrieNode>>>,

    pub(crate) value: Option<TrieNodeValueTypes>
}

impl TrieNode {

    // Create a TrieNode with no children and no value.
    pub fn empty() -> Self {
        TrieNode {
            children: None,
            value: None
        }
    }

    pub fn new(value: Option<TrieNodeValueTypes>, children: Option<HashMap<char, Arc<TrieNode>>>) -> Self {
        TrieNode {
            children,
            value
        }
    }

    pub fn new_with_children(children: HashMap<char, Arc<TrieNode>>) -> Self {
        TrieNode {
            children: Some(children),
            value: None
        }
    }

    pub fn new_with_value(value: TrieNodeValueTypes) -> Self {
        TrieNode {
            children: None,
            value: Some(value)
        }
    }

    #[cfg(test)]
    pub(crate) fn get_child_at_char(&self, key: char) -> Option<Arc<TrieNode>> {
        self.children.as_ref().and_then(|c| c.get(&key).cloned())
    }
}
