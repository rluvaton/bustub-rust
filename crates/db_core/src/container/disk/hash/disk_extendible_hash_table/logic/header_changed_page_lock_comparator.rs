use crate::buffer::AlwaysValidPageLockComparator;

pub type HeaderChangedPageLockComparator = AlwaysValidPageLockComparator;

// impl PageLockComparator for HeaderChangedPageLockComparator {
//     type CompareError = ();
//
//     fn new(before: &UnderlyingPage) -> Self {
//         // We know for sure the type of the before
//         Self {}
//     }
//
//     fn compare(self, after: &UnderlyingPage) -> Result<(), Self::CompareError> {
//         // We cannot assume anything about the data in the underlying page
//         let header = after.cast::<ExtendibleHashTableHeaderPage>();
//
//
//
//
//
//         return Ok(());
//     }
// }
