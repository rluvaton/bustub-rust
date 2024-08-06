use crate::trie_node::TrieNode;
use crate::trie_node_with_value::TrieNodeWithValue;

#[derive(Clone, Debug)]
pub enum TrieNodeType<T: Clone> {
    WithValue(TrieNodeWithValue<T>),
    WithoutValue(TrieNode<T>),
}
