// This file is what the students will be implementing

use std::collections::HashMap;

use crate::trie::Trie;
use crate::trie_node::TrieNode;
use crate::trie_node_type::TrieNodeType;
use crate::trie_node_with_value::TrieNodeWithValue;

impl<V: Clone + Sized> Trie<V> {
    // Get the value associated with the given key.
    // 1. If the key is not in the trie, return nullptr.
    // 2. If the key is in the trie but the type is mismatched, return nullptr.
    // 3. Otherwise, return the value.
    pub fn get<T: Clone + Sized>(&self, key: &str) -> Option<V> {
        return if let Some(root) = &self.root {
            Self::get_recursive(root, key)
        } else {
            None
        }
    }

    fn get_recursive<T: Clone + Sized>(node: &TrieNodeType<T>, key: &str) -> Option<T> {
        let is_last_char = key.len() == 0;

        if is_last_char {
            return match node {
                TrieNodeType::WithValue(n) => Some(n.value.clone()),
                TrieNodeType::WithoutValue(_) => None
            };
        }

        let mut children: Option<HashMap<char, TrieNodeType<T>>>;

        match node {
            TrieNodeType::WithValue(c) => {
                // TODO - remove clone
                children = c.children.clone();
            }
            TrieNodeType::WithoutValue(c) => {
                // TODO - remove clone
                children = c.children.clone();
            }
        }

        if children.is_none() {
            return None;
        }

        let children = children.unwrap();

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
    pub fn put<T: Clone + Sized>(&self, key: &str, value: V) -> Self {
        let new_root = Self::put_recursive::<V>(self.root.clone(), key, value);

        let new_trie = Trie::new(new_root);

        return new_trie;
    }


    fn put_recursive<T: Clone + Sized>(possible_node: Option<TrieNodeType<T>>, key: &str, value: T) -> TrieNodeType<T> {
        let is_last_char = key.len() as isize - 1 <= 0;

        if is_last_char {
            let mut children: Option<HashMap<char, TrieNodeType<T>>> = None;

            if let Some(c) = possible_node {
                children = match c {
                    TrieNodeType::WithValue(n) => n.children,
                    TrieNodeType::WithoutValue(n) => n.children,
                }
            }

            return TrieNodeType::WithValue(TrieNodeWithValue::<T>::new(children, value));
        }

        let mut new_node: TrieNodeType<T>;

        match possible_node {
            None => {
                new_node = TrieNodeType::WithoutValue(
                    TrieNode::new(
                        HashMap::new()
                    )
                )
            }
            Some(n) => {
                new_node = n.clone()
            }
        }

        let next_char = key.chars().nth(1).expect("Must have another char");
        let mut possible_child: Option<TrieNodeType<T>> = None;

        match &new_node {
            TrieNodeType::WithValue(n) => {
                // TODO - remove clone
                if let Some(children) = n.children.clone() {
                    if children.contains_key(&next_char) {
                        possible_child = Some(children.get(&next_char).unwrap().clone());
                    }
                }
            }
            TrieNodeType::WithoutValue(n) => {
                // TODO - remove clone
                if let Some(children) = n.children.clone() {
                    if children.contains_key(&next_char) {
                        possible_child = Some(children.get(&next_char).unwrap().clone());
                    }
                }
            }
        }

        let child = Self::put_recursive(
            possible_child,
            &key[1..],
            value
        );

        match new_node {
            TrieNodeType::WithValue(ref n) => {
                // TODO - remove clone

                n.children.clone().expect("Must have children").insert(next_char, child);
            }
            TrieNodeType::WithoutValue(ref n) => {
                // TODO - remove clone

                n.children.clone().expect("Must have children").insert(next_char, child);
            }
        }

        return new_node;
    }

    // Remove the key from the trie. If the key does not exist, return the original trie.
    // Otherwise, returns the new trie.
    pub fn remove(&self, key: &str) -> Self {
        unimplemented!()
    }
}
