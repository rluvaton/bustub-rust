use crate::{DBTypeIdImpl, Value, run_on_impl};
use std::cmp::Ordering;

impl PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        run_on_impl!(self.value, lhs, &lhs == rhs)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        run_on_impl!(self.value, lhs, lhs.partial_cmp(other))
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO - add to error message the current type and the other type
        run_on_impl!(self.value, lhs, lhs.partial_cmp(other)).expect("Should be able to compare")
    }
}
