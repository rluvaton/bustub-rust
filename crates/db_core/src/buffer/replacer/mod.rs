mod traits;
mod lru_k_replacer;

pub(crate) use traits::{Replacer};

pub use lru_k_replacer::{LRUKReplacer};
