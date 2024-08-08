// This file is what the students will be implementing

use std::borrow::Cow;
use std::collections::HashMap;

use crate::trie::Trie;
use crate::trie_node::TrieNode;
use crate::trie_node_type::TrieNodeType;
use crate::trie_node_value_types::TrieNodeValueTypes;
use crate::trie_node_with_value::TrieNodeWithValue;

enum RemoveResult {
    NotFound,

    // If node has children
    ReplaceNode(TrieNodeType),

    // Remove node completely, if the found node has no children
    Remove,
}

impl Trie {
    // Get the value associated with the given key.
    // 1. If the key is not in the trie, return nullptr.
    // 2. If the key is in the trie but the type is mismatched, return nullptr.
    // 3. Otherwise, return the value.
    pub fn get(self: &Trie, key: &str) -> Option<&TrieNodeValueTypes> {
        return if let Some(root) = &self.root {
            Self::get_recursive(root, key)
        } else {
            None
        };
    }

    fn get_recursive<'a>(node: &'a TrieNodeType, key: &str) -> Option<&'a TrieNodeValueTypes> {
        let is_last_char = key.len() == 0;

        if is_last_char {
            return node.get_value();
        }


        let children: &Option<HashMap<char, TrieNodeType>> = node.get_children();

        if children.is_none() {
            return None;
        }

        let children = children.as_ref().unwrap();

        let c = key.chars().next().unwrap();

        if !children.contains_key(&c) {
            return None;
        }

        return Self::get_recursive(
            children.get(&c).unwrap(),

            // remove first char
            &key[1..],
        );
    }

    // Put a new key-value pair into the trie. If the key already exists, overwrite the value.
    // Returns the new trie.
    pub fn put(&self, key: &str, value: TrieNodeValueTypes) -> Cow<'_, Self> {
        let new_root = Self::put_recursive(self.root.as_ref(), key, value);

        let new_trie = Trie::new(new_root);

        return new_trie;
    }


    fn put_recursive(possible_node: Option<&TrieNodeType>, key: &str, value: TrieNodeValueTypes) -> TrieNodeType {
        let is_last_char = key.len() == 0;

        if is_last_char {
            let children: Option<HashMap<char, TrieNodeType>> = possible_node
                // Clone children so the reference won't be saved
                .and_then(|c| c.get_children().clone());


            return TrieNodeType::WithValue(TrieNodeWithValue::new(children, value));
        }

        let mut new_node: TrieNodeType;

        match possible_node {
            None => {
                new_node = TrieNodeType::WithoutValue(
                    TrieNode::new(
                        HashMap::new()
                    )
                )
            }
            Some(n) => {
                new_node = n.clone();
                new_node.init_children_if_missing()
            }
        }

        let next_char = key.chars().nth(0).expect("Must have first char");
        let possible_child: Option<&TrieNodeType> = new_node.get_child_at_char(next_char);

        let child = Self::put_recursive(
            possible_child,
            &key[1..],
            value,
        );

        let children = new_node.get_children_mut().as_mut().unwrap();

        children.insert(next_char, child);

        return new_node;
    }

    // Remove the key from the trie. If the key does not exist, return the original trie.
    // Otherwise, returns the new trie.
    pub fn remove(&self, key: &str) -> Cow<'_, Self> {
        if self.root.is_none() {
            // If not found return the same trie
            return Cow::Borrowed(self);
        }

        let remove_result = Self::remove_recursive(self.root.as_ref().unwrap(), key);

        return match remove_result {
            RemoveResult::NotFound => {
                // If not found return the same trie
                Cow::Borrowed(self)
            }
            RemoveResult::ReplaceNode(new_root) => {
                // Replace root, mean that node found but has children so need to keep
                let new_trie = Trie::new(new_root);

                new_trie
            }
            RemoveResult::Remove => {
                // Remove the node completely
                let new_trie = Trie::create_empty();

                new_trie
            }
        }
    }

    fn remove_recursive(node: &TrieNodeType, key: &str) -> RemoveResult {
        let is_last_char = key.len() == 0;

        if is_last_char {
            // If not value node than it's not found
            if !node.is_value_node() {
                return RemoveResult::NotFound;
            }

            // If node is value node and has children then just replace the node to be node without value
            // and return new node
            if node.has_children() {
                return RemoveResult::ReplaceNode(node.change_to_without_value());
            }

            // If value node and no children than need to remove this node
            return RemoveResult::Remove;
        }

        let children: &Option<HashMap<char, TrieNodeType>> = node.get_children();

        // key not found
        if children.is_none() {
            return RemoveResult::NotFound;
        }

        let children = children.as_ref().expect("Must have children as we checked before");

        let c = key.chars().next().expect("Must have first char");

        if !children.contains_key(&c) {
            return RemoveResult::NotFound;
        }


        let child: &TrieNodeType = children.get(&c).expect("Must have child");

        let remove_result = Self::remove_recursive(
            child,
            &key[1..],
        );

        return match remove_result {
            RemoveResult::NotFound => {
                RemoveResult::NotFound
            }
            RemoveResult::ReplaceNode(new_child) => {
                // Replacing the node
                let mut new_node = node.clone();
                new_node.get_children_mut().as_mut().unwrap().insert(c, new_child);

                RemoveResult::ReplaceNode(new_node)
            }
            RemoveResult::Remove => {
                // If the next char is the only char in children, should remove this node as well
                if children.len() == 1 {

                    // If not value node, then just remove this node as well
                    if !node.is_value_node() {
                        return RemoveResult::Remove;
                    }

                    // if value node, then need to remove the children
                    return RemoveResult::ReplaceNode(node.clone_to_be_without_children());
                }

                // Need to remove the node from the children
                // should only clone the value if has + children map without the actual children themselves
                let mut new_node = node.clone();

                // unwrap as we know for sure the children exist
                new_node.get_children_mut().as_mut().unwrap().remove(&c);

                RemoveResult::ReplaceNode(new_node)
            }
        };
    }
}
