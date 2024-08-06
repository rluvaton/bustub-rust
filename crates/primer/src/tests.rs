
#[cfg(test)]
mod tests {
    use crate::trie::Trie;
    use super::*;

    #[test]
    fn basic_put() {
        let trie = Trie::<u32>::create_empty();

        let trie = trie.put::<u32>("test-int", 233);
        let trie = trie.put::<u64>("test-int2", 23333333);
        let trie = trie.put::<String>("test-string", "test".to_string());
        let trie = trie.put::<String>("", "empty-key".to_string());
    }
}
