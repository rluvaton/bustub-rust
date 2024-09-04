pub enum AccessType {
    Unknown,
    Lookup,
    Scan,
    Index
}

impl Default for AccessType {
    fn default() -> Self {
        AccessType::Unknown
    }
}
