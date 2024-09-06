use std::fmt::Display;

// TODO - should this be in common?
pub trait PageKey: Sized + Clone + Display {}
pub trait PageValue: Sized + Clone + Display + PartialEq {}
