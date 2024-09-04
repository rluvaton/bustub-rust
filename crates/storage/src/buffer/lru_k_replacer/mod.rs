mod tests;
mod access_type;
mod lru_k_node;
mod counter;
mod lru_k_replacer;
mod single_thread_impl;


// For single thread
pub use single_thread_impl::{LRUKReplacerImpl};

// Wrapper around the impl for multi thread
pub use lru_k_replacer::LRUKReplacer;

pub use access_type::AccessType;
