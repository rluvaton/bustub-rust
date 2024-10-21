use std::cmp::Ordering;
use std::marker::PhantomData;

pub trait Comparator<Item>: Clone {
    fn cmp(&self, lhs: &Item, rhs: &Item) -> Ordering;
}

/// Wrapper for implementing `Comparator` for all types that implement `Clone` and `Ord`
#[derive(Default, Clone)]
pub struct OrdComparator<T: Clone + Ord> {
    phantom_data: PhantomData<T>
}

impl<T: Clone + Ord> Comparator<T> for OrdComparator<T> {
    fn cmp(&self, lhs: &T, rhs: &T) -> Ordering {
        lhs.cmp(rhs)
    }
}

// Common Comparators
pub type I8Comparator = OrdComparator<i8>;
pub type U8Comparator = OrdComparator<u8>;

pub type I16Comparator = OrdComparator<i16>;
pub type U16Comparator = OrdComparator<u16>;

pub type I32Comparator = OrdComparator<i32>;
pub type U32Comparator = OrdComparator<u32>;

pub type I64Comparator = OrdComparator<i64>;
pub type U64Comparator = OrdComparator<u64>;

pub type I128Comparator = OrdComparator<i128>;
pub type U128Comparator = OrdComparator<u128>;


