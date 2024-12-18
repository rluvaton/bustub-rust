// This module is for splitting each operation to own file for better readability

mod lookup;
mod insert;
mod remove;
mod update;
mod delete_completely;

pub use lookup::LookupError;
pub use insert::InsertionError;
pub use remove::RemoveError;
pub use update::UpdateError;
