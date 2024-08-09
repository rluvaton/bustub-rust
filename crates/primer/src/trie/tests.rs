#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::trie::trie::Trie;

    #[test]
    fn basic_put() {
        let trie = Trie::create_empty();

        let trie = trie.put("test-int", 233u32.into());
        let trie = trie.put("test-int2", 23333333u32.into());
        let trie = trie.put("test-string", "test".to_string().into());
        let _trie = trie.put("", "empty-key".to_string().into());
    }

    #[test]
    fn structure_check() {
        let trie = Trie::create_empty();

        // Put something
        let trie = trie.put("t", 233.into());
        let v = trie.get("t");

        assert_eq!(v, Some(&233.into()));

        // Ensure the trie is the same representation of the writeup
        // (Some students were using '\0' as the terminator in previous semesters)
        let root = trie.root.clone().expect("Must have root");

        let children = root.children.as_ref().expect("Must have children on root");

        assert_eq!(children.len(), 1);

        let child = children.get(&'t').unwrap();
        assert_eq!(child.children, None);


        // The original tests has the following which does not make sense as we only inserted t
        // ASSERT_EQ(root->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.at('e')->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.at('e')->children_.at('s')->children_.size(), 1);
        // ASSERT_EQ(root->children_.at('t')->children_.at('e')->children_.at('s')->children_.at('t')->children_.size(), 0);
        // ASSERT_TRUE(root->children_.at('t')->children_.at('e')->children_.at('s')->children_.at('t')->is_value_node_);
    }

    #[test]
    fn basic_put_get() {
        let trie = Trie::create_empty();

        // Put something
        let trie = trie.put("test", 233.into());
        assert_eq!(trie.get("test"), Some(&233.into()));

        // Put something else
        let trie = trie.put("test", 23333333.into());
        assert_eq!(trie.get("test"), Some(&23333333.into()));

        // Overwrite with another type
        let trie = trie.put("test", "23333333".to_string().into());
        assert_eq!(trie.get("test"), Some(&"23333333".to_string().into()));

        // Get something that doesn't exist
        assert_eq!(trie.get("test-2333"), None);

        // Put something at root
        let trie = trie.put("", "empty-key".to_string().into());
        assert_eq!(trie.get(""), Some(&"empty-key".to_string().into()));
    }

    #[test]
    fn put_get_one_path() {
        let trie = Trie::create_empty();

        let trie = trie.put("111", 111.into());
        let trie = trie.put("11", 11.into());
        let trie = trie.put("1111", 1111.into());
        let trie = trie.put("11", 22.into());

        assert_eq!(trie.get("11"), Some(&22.into()));
        assert_eq!(trie.get("111"), Some(&111.into()));
        assert_eq!(trie.get("1111"), Some(&1111.into()));
    }

    #[test]
    fn basic_remove_1() {
        let trie = Trie::create_empty();

        // Put something
        let trie = trie.put("test", 2333.into());
        assert_eq!(trie.get("test"), Some(&2333.into()));
        let trie = trie.put("te", 23.into());
        assert_eq!(trie.get("te"), Some(&23.into()));
        let trie = trie.put("tes", 233.into());
        assert_eq!(trie.get("tes"), Some(&233.into()));

        // Delete something
        let trie = trie.remove("test");
        let trie = trie.remove("tes");
        let trie = trie.remove("te");

        assert_eq!(trie.get("te"), None);
        assert_eq!(trie.get("tes"), None);
        assert_eq!(trie.get("test"), None);
    }

    #[test]
    fn basic_remove_2() {
        let trie = Trie::create_empty();

        // Put something
        let trie = trie.put("test", 2333.into());
        assert_eq!(trie.get("test"), Some(&2333.into()));
        let trie = trie.put("te", 23.into());
        assert_eq!(trie.get("te"), Some(&23.into()));
        let trie = trie.put("tes", 233.into());
        assert_eq!(trie.get("tes"), Some(&233.into()));
        let trie = trie.put("", 123.into());
        assert_eq!(trie.get(""), Some(&123.into()));

        // Delete something
        let trie = trie.remove("");
        let trie = trie.remove("te");
        let trie = trie.remove("tes");
        let trie = trie.remove("test");

        assert_eq!(trie.get(""), None);
        assert_eq!(trie.get("te"), None);
        assert_eq!(trie.get("tes"), None);
        assert_eq!(trie.get("test"), None);
    }

    #[test]
    fn remove_free() {
        let trie = Trie::create_empty();

        let trie = trie.put("test", 2333.into());
        let trie = trie.put("te", 23.into());
        let trie = trie.put("tes", 233.into());

        let trie = trie.remove("tes");
        let trie = trie.remove("test");

        let trie_root = trie.root.as_ref().expect("must have root");
        let trie_node_at_t = trie_root.get_child_at_char('t').expect("must have child t");
        let trie_node_at_e = trie_node_at_t.get_child_at_char('e').expect("must have child e");

        assert_eq!(trie_node_at_e.children, None);

        let trie = trie.remove("te");

        assert_eq!(trie.root, None);
    }

    // No mismatch type test as the original project as this cannot happen in the current impl

    #[test]
    fn copy_on_write_1() {
        let empty_trie = Trie::create_empty();

        // Put something
        let trie1 = empty_trie.put("test", 2333.into());
        let trie2 = trie1.put("te", 23.into());
        let trie3 = trie2.put("tes", 233.into());

        // Delete something
        let trie4 = trie3.remove("te");
        let trie5 = trie3.remove("tes");
        let trie6 = trie3.remove("test");

        // Check each snapshot
        assert_eq!(trie3.get("te"), Some(&23.into()));
        assert_eq!(trie3.get("tes"), Some(&233.into()));
        assert_eq!(trie3.get("test"), Some(&2333.into()));

        assert_eq!(trie4.get("te"), None);
        assert_eq!(trie4.get("tes"), Some(&233.into()));
        assert_eq!(trie4.get("test"), Some(&2333.into()));

        assert_eq!(trie5.get("te"), Some(&23.into()));
        assert_eq!(trie5.get("tes"), None);
        assert_eq!(trie5.get("test"), Some(&2333.into()));

        assert_eq!(trie6.get("te"), Some(&23.into()));
        assert_eq!(trie6.get("tes"), Some(&233.into()));
        assert_eq!(trie6.get("test"), None);
    }

    #[test]
    fn copy_on_write_2() {
        let empty_trie = Trie::create_empty();

        // Put something
        let trie1 = empty_trie.put("test", 2333.into());
        let trie2 = trie1.put("te", 23.into());
        let trie3 = trie2.put("tes", 233.into());

        // Delete something
        let trie4 = trie3.put("te", "23".to_string().into());
        let trie5 = trie3.put("tes", "233".to_string().into());
        let trie6 = trie3.put("test", "2333".to_string().into());

        // Check each snapshot
        assert_eq!(trie3.get("te"), Some(&23.into()));
        assert_eq!(trie3.get("tes"), Some(&233.into()));
        assert_eq!(trie3.get("test"), Some(&2333.into()));

        assert_eq!(trie4.get("te"), Some(&"23".to_string().into()));
        assert_eq!(trie4.get("tes"), Some(&233.into()));
        assert_eq!(trie4.get("test"), Some(&2333.into()));

        assert_eq!(trie5.get("te"), Some(&23.into()));
        assert_eq!(trie5.get("tes"), Some(&"233".to_string().into()));
        assert_eq!(trie5.get("test"), Some(&2333.into()));

        assert_eq!(trie6.get("te"), Some(&23.into()));
        assert_eq!(trie6.get("tes"), Some(&233.into()));
        assert_eq!(trie6.get("test"), Some(&"2333".to_string().into()));
    }

    #[test]
    fn copy_on_write_3() {
        let empty_trie = Trie::create_empty();

        // Put something
        let trie1 = empty_trie.put("test", 2333.into());
        let trie2 = trie1.put("te", 23.into());
        let trie3 = trie2.put("", 233.into());

        // Delete something
        let trie4 = trie3.put("te", "23".to_string().into());
        let trie5 = trie3.put("", "233".to_string().into());
        let trie6 = trie3.put("test", "2333".to_string().into());

        // Check each snapshot
        assert_eq!(trie3.get("te"), Some(&23.into()));
        assert_eq!(trie3.get(""), Some(&233.into()));
        assert_eq!(trie3.get("test"), Some(&2333.into()));

        assert_eq!(trie4.get("te"), Some(&"23".to_string().into()));
        assert_eq!(trie4.get(""), Some(&233.into()));
        assert_eq!(trie4.get("test"), Some(&2333.into()));

        assert_eq!(trie5.get("te"), Some(&23.into()));
        assert_eq!(trie5.get(""), Some(&"233".to_string().into()));
        assert_eq!(trie5.get("test"), Some(&2333.into()));

        assert_eq!(trie6.get("te"), Some(&23.into()));
        assert_eq!(trie6.get(""), Some(&233.into()));
        assert_eq!(trie6.get("test"), Some(&"2333".to_string().into()));
    }

    #[test]
    fn mixed() {
        // TODO - this is very slow, probably because all of the cloning
        let mut trie = Trie::create_empty();

        for i in 0..23333 {
            let key = format!("{:#05}", i);
            let value = format!("value-{:#08}", i);

            // I need each put to transfer ownership of the entire trie
            trie = trie.put(key.as_str(), value.into());
        }

        let trie_full: Rc<Trie> = Rc::clone(&trie);

        for i in (0..23333).step_by(2) {
            let key = format!("{:#05}", i);
            let value = format!("new-value-{:#08}", i);

            trie = trie.put(key.as_str(), value.into());
        }

        let trie_override: Rc<Trie> = Rc::clone(&trie);

        for i in (0..23333).step_by(3) {
            let key = format!("{:#05}", i);

            trie = trie.remove(key.as_str());
        }

        let trie_final: Rc<Trie> = Rc::clone(&trie);

        // verify trie_full
        for i in 0..23333 {
            let key = format!("{:#05}", i);
            let value = format!("value-{:#08}", i);

            assert_eq!(trie_full.get(key.as_str()), Some(&value.into()));
        }

        // verify trie_override
        for i in 0..23333 {
            let key = format!("{:#05}", i);
            if i % 2 == 0 {
                let value = format!("new-value-{:#08}", i);
                assert_eq!(trie_override.get(key.as_str()), Some(&value.into()));
            } else {
                let value = format!("value-{:#08}", i);
                assert_eq!(trie_override.get(key.as_str()), Some(&value.into()));
            }
        }

        // verify final trie
        for i in 0..23333 {
            let key = format!("{:#05}", i);
            if i % 3 == 0 {
                assert_eq!(trie_final.get(key.as_str()), None);
            } else if i % 2 == 0 {
                let value = format!("new-value-{:#08}", i);
                assert_eq!(trie_final.get(key.as_str()), Some(&value.into()));
            } else {
                let value = format!("value-{:#08}", i);
                assert_eq!(trie_final.get(key.as_str()), Some(&value.into()));
            }
        }
    }

    #[test]
    fn pointer_stability() {
        let trie = Trie::create_empty();

        let trie = trie.put("test", 2333.into());

        let ptr_before = trie.get("test").unwrap();

        let trie = trie.put("tes", 233.into());
        let trie = trie.put("te", 23.into());

        let ptr_after = trie.get("test").unwrap();

        assert_eq!(std::ptr::eq(ptr_before, ptr_after), true, "Should point to the same location - not cloned");
    }
}
