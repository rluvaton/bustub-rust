use std::collections::HashMap;
use std::rc::Rc;
use crate::trie::trie_node::TrieNode;
use crate::trie::trie_node_value_types::TrieNodeValueTypes;
use crate::trie::trie_node_with_value::TrieNodeWithValue;

#[derive(Clone, Debug, PartialEq)]
pub enum TrieNodeType {
    WithValue(TrieNodeWithValue),
    WithoutValue(TrieNode),
}

impl TrieNodeType {
    pub(crate) fn has_children(&self) -> bool {
        match &self {
            TrieNodeType::WithValue(c) => {
                c.children.is_some()
            }
            TrieNodeType::WithoutValue(c) => {
                c.children.is_some()
            }
        }
    }
    pub(crate) fn get_children(&self) -> &Option<HashMap<char, Rc<TrieNodeType>>> {
        match &self {
            TrieNodeType::WithValue(c) => {
                &c.children
            }
            TrieNodeType::WithoutValue(c) => {
                &c.children
            }
        }
    }

    pub(crate) fn get_children_mut(&mut self) -> &mut Option<HashMap<char, Rc<TrieNodeType>>> {
        match self {
            TrieNodeType::WithValue(c) => {
                &mut c.children
            }
            TrieNodeType::WithoutValue(c) => {
                &mut c.children
            }
        }
    }

    /// Will return new instance of trie node type without children
    pub(crate) fn clone_to_be_without_children(&self) -> Self {
        return match self {
            TrieNodeType::WithValue(node) => TrieNodeType::WithValue(TrieNodeWithValue::with_value(node.value.clone())),
            TrieNodeType::WithoutValue(_) => TrieNodeType::WithoutValue(TrieNode::empty()),
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

    pub(crate) fn get_child_at_char(&self, c: char) -> Option<Rc<TrieNodeType>> {
        let children = self.get_children();

        if let Some(children) = children {
            if children.contains_key(&c) {
                return children.get(&c).cloned();
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

    /// this should be used when knowing for sure that current node is value node
    pub(crate) fn change_to_without_value(&self) -> Self {
        match self {
            TrieNodeType::WithValue(node) => {
                TrieNodeType::WithoutValue(
                    node.into()
                )
            }
            _ => {
                unreachable!("Must be node with value")
            }
        }
    }

    // pub(crate) fn clone_while_keeping_children_ref
}
