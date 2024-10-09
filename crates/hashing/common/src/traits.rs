use common::{PageKey};
use std::hash::{DefaultHasher as DefaultRustHasher, Hasher};

pub trait KeyHasher {
     fn hash_key<Key: PageKey>(key: &Key) -> u64;
}


pub struct DefaultKeyHasher;

impl KeyHasher for DefaultKeyHasher {
    fn hash_key<Key: PageKey>(key: &Key) -> u64 {
        // TODO = replace with MurmurHash3 with fixed seed
        let mut hasher = DefaultRustHasher::new();

        key.hash(&mut hasher);

        hasher.finish()
    }
}

pub struct U64IdentityKeyHasher;

impl KeyHasher for U64IdentityKeyHasher {
    fn hash_key<Key: PageKey>(key: &Key) -> u64 {
        let p = key as *const Key;
        let n = p as *const u64;

        unsafe {
            *n
        }
    }
}
