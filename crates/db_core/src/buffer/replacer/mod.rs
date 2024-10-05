mod traits;
mod lru_k_replacer;
mod access_type;

pub(crate) use traits::{Replacer, ThreadSafeReplacer};

pub use lru_k_replacer::{LRUKReplacer, LRUKReplacerImpl};
pub use access_type::AccessType;
