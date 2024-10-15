use std::fmt::{Debug, Display};
use std::hash::Hash;

// TODO - should this be in common?
pub trait PageKey: Sized + Clone + Debug + Hash {}
pub trait PageValue: Sized + Clone + Display + Debug + PartialEq {}

macro_rules! page_key_value_impl {
    ($($t:ty)+) => ($(
        impl PageKey for $t {}
        impl PageValue for $t {}
    )+)
}

page_key_value_impl! { i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 }
