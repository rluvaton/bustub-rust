
#[cfg(test)]
mod tests {
    use crate::trie::Trie;
    use super::*;

    #[test]
    fn basic_put() {
        let trie = Trie::create_empty();

        let trie = trie.put("test-int", 233u32.into());
        let trie = trie.put("test-int2", 23333333u32.into());
        // let trie = trie.put::<String>("test-string", 123);
        let trie = trie.put("test-string", "test".to_string().into());
        // let trie = trie.put::<String>("", 0);
        let trie = trie.put("", "empty-key".to_string().into());
    }

    #[test]
    fn structure_check() {
        let trie = Trie::create_empty();

        // Put something
        let trie = trie.put("t", 233u32.into());
        let v = trie.get("t");

        assert_eq!(v, Some(&233u32.into()));

        // Ensure the trie is the same representation of the writeup
        // (Some students were using '\0' as the terminator in previous semesters)
        let root = trie.root.expect("Must have root");

        let children = root.get_children().as_ref().expect("Must have children on root");

        assert_eq!(children.len(), 1);

        let child = children.get(&'t').unwrap();
        assert_eq!(child.get_children(), &None);

        // TODO - add rest of checks

        // The original tests has the following which does not make sense as we only inserted t
        // ASSERT_EQ(root->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.at('e')->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.at('e')->children_.at('s')->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.at('e')->children_.at('s')->children_.at('t')->children_.size(), 0);
        // ASSERT_TRUE(root->children_.at('t')->children_.at('e')->children_.at('s')->children_.at('t')->is_value_node_);

    }
}
