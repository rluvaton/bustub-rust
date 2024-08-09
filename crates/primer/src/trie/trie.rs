use std::rc::Rc;
use crate::trie::trie_node_type::TrieNodeType;

#[derive(Clone, Debug)]
pub struct Trie {

    // The root of the trie.
    pub(crate) root: Option<Rc<TrieNodeType>>,
}

impl<'a> Trie {

    // Create an empty trie.
    pub fn create_empty() -> Rc<Self> {
        Rc::new(Trie {
            root: None,
        })
    }

    // Create a new trie with the given root.
    pub fn new(root: Rc<TrieNodeType>) -> Rc<Self> {
        Rc::new(Trie {
            root: Some(Rc::clone(&root)),
        })
    }
}
