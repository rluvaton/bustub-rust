// This file is what the students will be implementing

use crate::trie::trie::Trie;
use crate::trie::trie_node::TrieNode;
use crate::trie::trie_node_value_types::TrieNodeValueTypes;
use std::collections::HashMap;
use std::sync::Arc;

enum RemoveResult {
    NotFound,

    // If node has children
    ReplaceNode(Arc<TrieNode>),

    // Remove node completely, if the found node has no children
    Remove,
}

impl Trie {
    // Get the value associated with the given key.
    // 1. If the key is not in the trie, return nullptr.
    // 2. If the key is in the trie but the type is mismatched, return nullptr.
    // 3. Otherwise, return the value.
    pub fn get(self: &Trie, key: &str) -> Option<&TrieNodeValueTypes> {
        if let Some(root) = &self.root {
            Self::get_recursive(root, key)
        } else {
            None
        }
    }

    fn get_recursive<'a>(node: &'a TrieNode, key: &str) -> Option<&'a TrieNodeValueTypes> {
        let is_last_char = key.len() == 0;

        if is_last_char {
            return node.value.as_ref();
        }


        let children: &Option<HashMap<char, Arc<TrieNode>>> = &node.children;

        if children.is_none() {
            return None;
        }

        let children = children.as_ref()?;

        let c = key.chars().next()?;

        if !children.contains_key(&c) {
            return None;
        }

        Self::get_recursive(
            children.get(&c)?,

            // remove first char
            &key[1..],
        )
    }

    // Put a new key-value pair into the trie. If the key already exists, overwrite the value.
    // Returns the new trie.
    pub fn put(&self, key: &str, value: TrieNodeValueTypes) -> Arc<Self> {
        let new_root = Self::put_recursive(self.root.clone(), key, value);

        let new_trie = Trie::new(new_root);

        new_trie
    }

    fn put_recursive(possible_node: Option<Arc<TrieNode>>, key: &str, value: TrieNodeValueTypes) -> Arc<TrieNode> {
        let is_last_char = key.len() == 0;

        if is_last_char {
            let children: Option<HashMap<char, Arc<TrieNode>>> = possible_node
                // Clone children so the reference won't be saved
                .and_then(|c| c.children.clone());


            return Arc::new(TrieNode::new(Some(value), children));
        }

        let mut new_node: TrieNode;
        // let mut new_node: Rc<TrieNodeType>;

        match possible_node {
            None => {
                new_node = TrieNode::new_with_children(
                        HashMap::new()
                    )

            }
            Some(n) => {
                new_node = n.as_ref().clone();
                if new_node.children.is_none() {
                    new_node.children = Some(HashMap::new())
                }
            }
        }

        let next_char = key.chars().nth(0).expect("Must have first char");
        let possible_child: Option<Arc<TrieNode>> = new_node.children.as_ref().and_then(|m| m.get(&next_char).cloned());

        let child = Self::put_recursive(
            possible_child,
            &key[1..],
            value,
        );

        let children = new_node.children.as_mut().unwrap();

        children.insert(next_char, child);

        Arc::new(new_node)
    }

    // Remove the key from the trie. If the key does not exist, return the original trie.
    // Otherwise, returns the new trie.
    pub fn remove(&self, key: &str) -> Arc<Self> {
        if self.root.is_none() {
            // If not found return the same trie
            return Arc::new(self.clone());
        }

        let this = self.clone();

        let remove_result = Self::remove_recursive(this.root.unwrap(), key);

        match remove_result {
            RemoveResult::NotFound => {
                // If not found return the same trie
                Arc::new(self.clone())
            }
            RemoveResult::ReplaceNode(new_root) => {
                // Replace root, mean that node found but has children so need to keep
                Trie::new(new_root)
            }
            RemoveResult::Remove => {
                // Remove the node completely
                Trie::create_empty()
            }
        }
    }

    fn remove_recursive(node: Arc<TrieNode>, key: &str) -> RemoveResult {
        let is_last_char = key.len() == 0;

        if is_last_char {
            // If not value node than it's not found
            if node.value.is_none() {
                return RemoveResult::NotFound;
            }

            // If node is value node and has children then just replace the node to be node without value
            // and return new node
            if node.children.is_some() {
                let new_node = TrieNode::new(None, node.children.clone());
                return RemoveResult::ReplaceNode(Arc::new(new_node));
            }

            // If value node and no children than need to remove this node
            return RemoveResult::Remove;
        }

        let children: &Option<HashMap<char, Arc<TrieNode>>> = &node.children;

        // key not found
        if children.is_none() {
            return RemoveResult::NotFound;
        }

        let children = children.as_ref().expect("Must have children as we checked before");

        let c = key.chars().next().expect("Must have first char");

        if !children.contains_key(&c) {
            return RemoveResult::NotFound;
        }

        let child: Arc<TrieNode> = Arc::clone(children.get(&c).expect("Must have child"));

        let remove_result = Self::remove_recursive(
            child,
            &key[1..],
        );

        match remove_result {
            RemoveResult::NotFound => {
                RemoveResult::NotFound
            }
            RemoveResult::ReplaceNode(new_child) => {
                // Replacing the node
                let mut tmp = node.clone();
                let new_node = Arc::make_mut(&mut tmp);
                new_node.children.as_mut().unwrap().insert(c, new_child);

                RemoveResult::ReplaceNode(Arc::new(new_node.clone()))
            }
            RemoveResult::Remove => {
                // If the next char is the only char in children, should remove this node as well
                if children.len() == 1 {
                    // If not value node, then just remove this node as well
                    if node.value.is_none() {
                        return RemoveResult::Remove;
                    }

                    // if value node, then need to remove the children
                    let new_node: TrieNode = TrieNode::new(node.value.clone(), None);
                    return RemoveResult::ReplaceNode(Arc::new(new_node));
                }

                // Need to remove the node from the children
                // should only clone the value if has + children map without the actual children themselves
                let mut tmp = node.clone();
                let new_node = Arc::make_mut(&mut tmp);

                // unwrap as we know for sure the children exist
                new_node.children.as_mut().unwrap().remove(&c);

                RemoveResult::ReplaceNode(Arc::new(new_node.clone()))
            }
        }
    }
}
