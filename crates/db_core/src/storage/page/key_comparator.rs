use std::cmp::Ordering;

pub trait Comparator<Item>: Clone {
    fn cmp(&self, lhs: &Item, rhs: &Item) -> Ordering;
}


#[cfg(test)]
pub mod test_util {
    use std::cmp::Ordering;
    use crate::storage::Comparator;

    #[derive(Clone)]
    pub struct U64Comparator;

    impl Comparator<u64> for U64Comparator {
        ///
        /// Function object returns true if lhs < rhs, used for trees
        ///
        fn cmp(&self, lhs: &u64, rhs: &u64) -> Ordering {
            lhs.cmp(rhs)
        }
    }
}
