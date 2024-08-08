use std::collections::HashMap;
use crate::trie_node::TrieNode;
use crate::trie_node_value_types::TrieNodeValueTypes;
use crate::trie_node_with_value::TrieNodeWithValue;

#[derive(Clone, Debug, PartialEq)]
pub enum TrieNodeType {
    WithValue(TrieNodeWithValue),
    WithoutValue(TrieNode),
}

impl TrieNodeType {
    pub(crate) fn get_children(&self) -> &Option<HashMap<char, TrieNodeType>> {
        match &self {
            TrieNodeType::WithValue(c) => {
                &c.children
            }
            TrieNodeType::WithoutValue(c) => {
                &c.children
            }
        }
    }

    pub(crate) fn get_children_mut(&mut self) -> &mut Option<HashMap<char, TrieNodeType>> {
        match self {
            TrieNodeType::WithValue(c) => {
                &mut c.children
            }
            TrieNodeType::WithoutValue(c) => {
                &mut c.children
            }
        }
    }

    pub(crate) fn is_value_node(&self) -> bool {
        match self {
            TrieNodeType::WithValue(_) => true,
            TrieNodeType::WithoutValue(_) => false,
        }
    }

    pub(crate) fn get_value(&self) -> Option<&TrieNodeValueTypes> {
        match self {
            TrieNodeType::WithValue(n) => Some(&n.value),
            TrieNodeType::WithoutValue(_) => None,
        }
    }

    pub(crate) fn get_child_at_char(&self, c: char) -> Option<&TrieNodeType> {
        let children = self.get_children();

        if let Some(children) = children {
            if (children.contains_key(&c)) {
                return children.get(&c);
            }
        }

        return None;
    }

    pub(crate) fn init_children_if_missing(&mut self) {
        match self {
            TrieNodeType::WithValue(c) => {
                if c.children.is_none() {
                    c.children = Some(HashMap::new());
                }
            }
            TrieNodeType::WithoutValue(c) => {
                if c.children.is_none() {
                    c.children = Some(HashMap::new());
                }
            }
        }
    }
}
