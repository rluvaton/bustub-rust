mod traits;
mod lru_k_replacer;
mod access_type;

pub(crate) use traits::{Replacer};

pub use lru_k_replacer::{LRUKReplacer};
pub use access_type::AccessType;
