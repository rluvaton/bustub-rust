use std::cmp::Ordering;

pub trait Comparator<Item> {
    fn cmp(&self, lhs: &Item, rhs: &Item) -> Ordering;
}
