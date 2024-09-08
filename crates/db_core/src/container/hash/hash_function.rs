use std::hash::{DefaultHasher, Hash, Hasher};

use common::PageKey;

pub struct HashFunction<Key: PageKey> {}

impl<Key: PageKey> HashFunction<Key> {
    pub fn get_hash(key: Key) -> u64 {
        // TODO = replace with MurmurHash3 with fixed seed
        let mut hasher = DefaultHasher::new();

        key.hash(&mut hasher);

        hasher.finish()
    }
}
