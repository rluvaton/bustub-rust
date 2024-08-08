
#[derive(Clone, Debug, PartialEq)]
pub enum TrieNodeValueTypes {
    I8(i8),
    U8(u8),

    I16(i16),
    U16(u16),

    I32(i32),
    U32(u32),

    I64(i64),
    U64(u64),

    I128(i128),
    U128(u128),

    F32(f32),
    F64(f64),

    Bool(bool),

    String(String),
}

impl From<u8> for TrieNodeValueTypes {
    fn from(value: u8) -> Self {
        TrieNodeValueTypes::U8(value)
    }
}

impl From<i8> for TrieNodeValueTypes {
    fn from(value: i8) -> Self {
        TrieNodeValueTypes::I8(value)
    }
}

impl From<u16> for TrieNodeValueTypes {
    fn from(value: u16) -> Self {
        TrieNodeValueTypes::U16(value)
    }
}

impl From<i16> for TrieNodeValueTypes {
    fn from(value: i16) -> Self {
        TrieNodeValueTypes::I16(value)
    }
}

impl From<u32> for TrieNodeValueTypes {
    fn from(value: u32) -> Self {
        TrieNodeValueTypes::U32(value)
    }
}

impl From<i32> for TrieNodeValueTypes {
    fn from(value: i32) -> Self {
        TrieNodeValueTypes::I32(value)
    }
}

impl From<u64> for TrieNodeValueTypes {
    fn from(value: u64) -> Self {
        TrieNodeValueTypes::U64(value)
    }
}

impl From<i64> for TrieNodeValueTypes {
    fn from(value: i64) -> Self {
        TrieNodeValueTypes::I64(value)
    }
}

impl From<f32> for TrieNodeValueTypes {
    fn from(value: f32) -> Self {
        TrieNodeValueTypes::F32(value)
    }
}

impl From<f64> for TrieNodeValueTypes {
    fn from(value: f64) -> Self {
        TrieNodeValueTypes::F64(value)
    }
}

impl From<bool> for TrieNodeValueTypes {
    fn from(value: bool) -> Self {
        TrieNodeValueTypes::Bool(value)
    }
}

impl From<String> for TrieNodeValueTypes {
    fn from(value: String) -> Self {
        TrieNodeValueTypes::String(value)
    }
}
