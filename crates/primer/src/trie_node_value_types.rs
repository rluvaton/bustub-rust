
#[derive(Clone, Debug, PartialEq)]
pub enum TrieNodeValueTypes {
    U32(u32),
    U64(u64),
    String(String),
}

impl From<u32> for TrieNodeValueTypes {
    fn from(value: u32) -> Self {
        TrieNodeValueTypes::U32(value)
    }
}

impl From<u64> for TrieNodeValueTypes {
    fn from(value: u64) -> Self {
        TrieNodeValueTypes::U64(value)
    }
}

impl From<String> for TrieNodeValueTypes {
    fn from(value: String) -> Self {
        TrieNodeValueTypes::String(value)
    }
}
