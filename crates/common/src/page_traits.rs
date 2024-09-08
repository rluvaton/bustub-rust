use std::fmt::Display;
use std::hash::Hash;

// TODO - should this be in common?
pub trait PageKey: Sized + Clone + Display + Hash {}
pub trait PageValue: Sized + Clone + Display + PartialEq {}
