use std::fmt::Display;
use std::hash::Hash;

// TODO - should this be in common?
pub trait PageKey: Sized + Copy + Clone + Display + Hash {}
pub trait PageValue: Sized + Copy + Clone + Display + PartialEq {}

// Useful for tests
impl PageKey for u64 {}
impl PageValue for u64 {}
