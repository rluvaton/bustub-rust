use std::hash::{DefaultHasher, Hash, Hasher};

use common::PageKey;

pub fn hash_key<Key: PageKey>(key: &Key) -> u64 {
    // TODO = replace with MurmurHash3 with fixed seed
    let mut hasher = DefaultHasher::new();

    key.hash(&mut hasher);

    hasher.finish()
}
