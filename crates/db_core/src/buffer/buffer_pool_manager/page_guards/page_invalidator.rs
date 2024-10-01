use std::convert::Infallible;
use std::fmt::Debug;
use crate::storage::UnderlyingPage;

/// This trait is used to check whether the page we hold is still the same page
/// this is used when upgrading read lock in a non-atomic way
pub trait PageLockComparator {
    type CompareError: Debug;

    /// Create comparator that the value before the lock drop is the provided value
    fn new(before: &UnderlyingPage) -> Self;

    /// Return error if the page data is not the same to the one return in the new function
    ///
    /// # Safety
    /// Consumers cannot assume the type of the underlying page
    fn compare(self, after: &UnderlyingPage) -> Result<(), Self::CompareError>;
}

#[derive(Debug)]
pub struct AlwaysValidPageLockComparator;

impl PageLockComparator for AlwaysValidPageLockComparator {
    type CompareError = Infallible;

    fn new(before: &UnderlyingPage) -> Self {
        AlwaysValidPageLockComparator
    }

    fn compare(self, after: &UnderlyingPage) -> Result<(), Self::CompareError> {
        Ok(())
    }
}
