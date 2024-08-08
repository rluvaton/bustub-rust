// This file is what the students will be implementing

use std::collections::HashMap;

use crate::trie::Trie;
use crate::trie_node::TrieNode;
use crate::trie_node_type::TrieNodeType;
use crate::trie_node_value_types::TrieNodeValueTypes;
use crate::trie_node_with_value::TrieNodeWithValue;

impl Trie {
    // Get the value associated with the given key.
    // 1. If the key is not in the trie, return nullptr.
    // 2. If the key is in the trie but the type is mismatched, return nullptr.
    // 3. Otherwise, return the value.
    pub fn get(&self, key: &str) -> Option<&TrieNodeValueTypes> {
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
    pub fn put(&self, key: &str, value: TrieNodeValueTypes) -> Self {
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

        let mut children = new_node.get_children_mut().as_mut().unwrap();

        children.insert(next_char, child);

        return new_node;
    }

    // Remove the key from the trie. If the key does not exist, return the original trie.
    // Otherwise, returns the new trie.
    pub fn remove(&self, key: &str) -> Self {
        unimplemented!()
    }
}
