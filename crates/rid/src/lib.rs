// This is in its own crate for avoiding circular dependencies and allow to move page id to own crate and still depend on common

mod rid;

pub use rid::RID;
